#![cfg(test)]

use bincode::{Decode, Encode};
use crate::error::ProtocolError;
use crate::frame::Frame;
use crate::prelude::*;

#[derive(Encode, Decode, Default)]
pub struct TestRequest {
    id: i64,
    name: String
}

impl Message for TestRequest {
    fn message_type() -> u16 {
        1
    }

    fn to_frame(&self, sequence: u32) -> Result<Frame, ProtocolError> {
        let payload = self.serialize()?;
        Ok(Frame::new(
            Self::message_type(),
            sequence,
            FrameFlags::HAS_PAYLOAD,
            payload,
        ))
    }

    fn from_frame(frame: Frame) -> Result<Self, ProtocolError> {
        if !frame.has_payload() {
            return Err(ProtocolError::ExpectedPayload);
        }
        Self::deserialize(frame.payload)
    }
}

#[derive(Debug, Clone, Encode, Decode, PartialEq)]
pub struct TestResponse {
    id: i64,
    message: String,
}

impl Message for TestResponse {
    fn message_type() -> u16 {
        2
    }

    fn to_frame(&self, sequence: u32) -> Result<Frame, ProtocolError> {
        let payload = self.serialize()?;
        Ok(Frame::new(
            Self::message_type(),
            sequence,
            FrameFlags::HAS_PAYLOAD,
            payload,
        ))
    }

    fn from_frame(frame: Frame) -> Result<Self, ProtocolError> {
        if !frame.has_payload() {
            return Err(ProtocolError::ExpectedPayload);
        }
        Self::deserialize(frame.payload)
    }
}

async fn test_handler(req: TestRequest) -> Result<TestResponse, ProtocolError> {
    Ok(TestResponse {
        id: req.id,
        message: format!("Hello {}", req.name),
    })
}

#[tokio::test]
async fn test() {
    let mut registry = MessageRegistry::new();
    
    registry.register(test_handler.handler(TestRequest::message_type(), TestResponse::message_type()));

    // Create a test request
    let request = TestRequest {
        id: 42,
        name: "Test".to_string(),
    };

    // Serialize the request to a frame
    let request_frame = request.to_frame(123).unwrap();

    // Process the frame through the registry
    let process_future = registry.process_frame(request_frame).unwrap();

    // Await the future to get a response frame
    let response_frame = process_future.await.unwrap();

    // Deserialize the response frame
    let response = TestResponse::from_frame(response_frame).unwrap();

    // Verify the response
    assert_eq!(response.id, 42);
    assert_eq!(response.message, "Hello Test");
}