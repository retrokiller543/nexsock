use std::io;
pub use crate::prelude::*;

/// Transport trait for sending and receiving frames
pub trait Transport {
    /// Send a frame over the transport
    async fn send_frame(&mut self, frame: Frame) -> io::Result<()>;

    /// Receive a frame from the transport
    async fn receive_frame(&mut self) -> io::Result<Frame>;
}