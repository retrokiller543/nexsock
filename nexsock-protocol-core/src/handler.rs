// src/nexsock-protocol/src/handler.rs
use std::collections::HashMap;
use std::future::Future;
use std::marker::PhantomData;
use std::pin::Pin;
use std::sync::Arc;
use bytes::Bytes;
use futures::future::BoxFuture;
use tokio::pin;
use crate::prelude::*;

/// A trait for extracting typed data from a request frame
pub trait FromRequest: Sized + Message {
    type Error: Into<ProtocolError> + std::error::Error;

    /// Extract and convert the payload into the concrete type
    fn from_request(payload: Bytes) -> Result<Self, Self::Error>;
}

/// Default FromRequest implementation for BinaryMessage types
impl<T: Message> FromRequest for T {
    type Error = ProtocolError;

    fn from_request(payload: Bytes) -> Result<Self, Self::Error> {
        Self::deserialize(payload)
    }
}

/// A trait for handlers that process requests and return responses
pub trait Handler<Req, Res> {
    type Future: Future<Output = Result<Res, ProtocolError>>;

    fn call(&self, request: Req) -> Self::Future;

    /// Get the message type this handler processes
    fn message_type(&self) -> u16;

    /// Get the response message type
    fn response_type(&self) -> u16;
}

/// Type alias for a function-based handler
pub type HandlerFn<F, Req, Res> = HandlerFunc<F, Req, Res>;

/// A wrapper for function-based handlers
pub struct HandlerFunc<F, Req, Res> {
    f: F,
    req_type: u16,
    res_type: u16,
    _req: PhantomData<Req>,
    _res: PhantomData<Res>,
}

impl<F, Req, Res, Fut> HandlerFunc<F, Req, Res>
where
    F: Fn(Req) -> Fut,
    Req: Message,
    Res: Message,
    Fut: Future<Output = Result<Res, ProtocolError>>,
{
    pub fn new(f: F, req_type: u16, res_type: u16) -> Self {
        Self {
            f,
            req_type,
            res_type,
            _req: PhantomData,
            _res: PhantomData,
        }
    }
}

impl<F, Req, Res, Fut> Handler<Req, Res> for HandlerFunc<F, Req, Res>
where
    F: Fn(Req) -> Fut,
    Req: Message,
    Res: Message,
    Fut: Future<Output = Result<Res, ProtocolError>>,
{
    type Future = Fut;

    fn call(&self, request: Req) -> Self::Future {
        (self.f)(request)
    }

    fn message_type(&self) -> u16 {
        self.req_type
    }

    fn response_type(&self) -> u16 {
        self.res_type
    }
}

// First, let's create a type-erased version of FrameProcessFuture
pub type BoxedFrameFuture<'a> = BoxFuture<'a, Result<Bytes, ProtocolError>> /*Pin<Box<dyn Future<Output = Result<Bytes, ProtocolError>> + Send + Unpin>>*/;

/// A future that will process a frame and produce a response frame
pub struct FrameProcessFuture<'a> {
    future: BoxedFrameFuture<'a>,
    response_type: u16,
    sequence: u32,
}

impl<'a> Future for FrameProcessFuture<'a> {
    type Output = Result<Frame, ProtocolError>;

    fn poll(mut self: Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> std::task::Poll<Self::Output> {
        let response_type = self.response_type;
        let sequence = self.sequence;

        // Poll the boxed future
        match Pin::new(&mut self.future).poll(cx) {
            std::task::Poll::Ready(Ok(payload)) => std::task::Poll::Ready(Ok(Frame::new(
                response_type,
                sequence,
                FrameFlags::HAS_PAYLOAD,
                payload,
            ))),
            std::task::Poll::Ready(Err(err)) => std::task::Poll::Ready(Err(err)),
            std::task::Poll::Pending => std::task::Poll::Pending,
        }
    }
}

// Then update the MessageRegistry to use this non-generic version
pub struct MessageRegistry<'a> {
    handlers: HashMap<u16, Box<dyn Fn(Bytes) -> Result<FrameProcessFuture<'a>, ProtocolError> + Send + Sync>>,
}

impl<'a> MessageRegistry<'a> {
    pub fn new() -> Self {
        Self {
            handlers: HashMap::new(),
        }
    }
    
    // In your register method, box the future
    pub fn register<H, Req, Res>(&mut self, handler: H) -> &mut Self
    where
        H: Handler<Req, Res> + Send + Sync + 'static,
        Req: FromRequest,
        Res: Message,
        H::Future: Future<Output = Result<Res, ProtocolError>> + Send + 'a,
    {
        let message_type = handler.message_type();
        let response_type = handler.response_type();
        let handler = Arc::new(handler);

        let handler_fn = Box::new(move |payload: Bytes| -> Result<FrameProcessFuture, ProtocolError> {
            // Extract the request type from the payload
            let request = Req::from_request(payload).map_err(Into::into)?;

            // Create a clone for the handler call
            let inner_handler = handler.clone();

            // Call the handler to get its future
            let response_future = inner_handler.call(request);

            // Box and type-erase the future
            let boxed_future: BoxedFrameFuture = Box::pin(async move {
                let response = response_future.await?;
                response.serialize()
            });

            Ok(FrameProcessFuture {
                future: boxed_future,
                response_type,
                sequence: 0, // Will be set when processing the frame
            })
        });

        self.handlers.insert(message_type, handler_fn);
        self
    }

    // The process_frame method can now return the non-generic FrameProcessFuture
    pub fn process_frame(&self, frame: Frame) -> Result<FrameProcessFuture, ProtocolError> {
        let message_type = frame.message_type;
        let sequence = frame.sequence;

        // Find a handler for this message type
        let handler = self.handlers.get(&message_type).ok_or_else(|| {
            ProtocolError::Io(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                format!("No handler for message type: {}", message_type)
            ))
        })?;

        // Process the frame with the handler
        let mut future = handler(frame.payload)?;

        // Set the sequence number from the original frame
        future.sequence = sequence;

        Ok(future)
    }
}

/// Extension trait for easier handler registration
pub trait HandlerExt<F, Req, Res, Fut>
where
    F: Fn(Req) -> Fut,
    Req: Message + FromRequest,
    Res: Message,
    Fut: Future<Output = Result<Res, ProtocolError>>,
{
    fn handler(self, req_type: u16, res_type: u16) -> HandlerFunc<F, Req, Res>;
}

impl<F, Req, Res, Fut> HandlerExt<F, Req, Res, Fut> for F
where
    F: Fn(Req) -> Fut,
    Req: Message + FromRequest,
    Res: Message,
    Fut: Future<Output = Result<Res, ProtocolError>>,
{
    fn handler(self, req_type: u16, res_type: u16) -> HandlerFunc<F, Req, Res> {
        HandlerFunc::new(self, req_type, res_type)
    }
}