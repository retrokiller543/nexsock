use crate::prelude::*;
use bytes::{Buf, BytesMut};
use std::io;
use tokio::io::{AsyncRead, AsyncReadExt, AsyncWrite, AsyncWriteExt};
pub use tokio_transport::*;

pub struct StreamTransport<R, W> {
    reader: R,
    writer: W,
}

impl<R, W> StreamTransport<R, W>
where
    R: AsyncRead + Unpin,
    W: AsyncWrite + Unpin,
{
    /// Create a new stream transport
    pub fn new(reader: R, writer: W) -> Self {
        Self { reader, writer }
    }
}

impl<R, W> Transport for StreamTransport<R, W>
where
    R: AsyncRead + Unpin,
    W: AsyncWrite + Unpin,
{
    async fn send_frame(&mut self, frame: Frame) -> io::Result<()> {
        let encoded = frame.encode()?;
        self.writer.write_all(&encoded).await?;
        self.writer.flush().await?;
        Ok(())
    }

    async fn receive_frame(&mut self) -> io::Result<Frame> {
        let header_size = Frame::header_size();
        let mut header_buf = BytesMut::with_capacity(header_size + 4);
        header_buf.resize(header_size + 4, 0);

        self.reader.read_exact(&mut header_buf).await?;
        
        let payload_len = (&header_buf[header_size..header_size+4]).get_u32_le() as usize;

        if payload_len > 0 {
            let mut payload_buf = BytesMut::with_capacity(payload_len);
            payload_buf.resize(payload_len, 0);
            self.reader.read_exact(&mut payload_buf).await?;
            
            header_buf.extend_from_slice(&payload_buf);
            Frame::decode(header_buf.freeze())
        } else {
            Frame::decode(header_buf.freeze())
        }
    }
}

#[cfg(feature = "tokio")]
pub mod tokio_transport {
    //! Exposes convenience methods to convert tokio streams into a [`StreamTransport]
    use cfg_if::cfg_if;

    cfg_if! {
        if #[cfg(unix)] {
            use super::*;
            use tokio::net::UnixStream as Stream;
            use tokio::net::TcpStream;
            
            pub fn from_stream(stream: Stream) -> impl Transport {
                let (read, write) = stream.into_split();
                StreamTransport::new(read, write)
            }
            
            pub fn from_tcp_stream(stream: TcpStream) -> impl Transport {
                let (read, write) = stream.into_split();
                StreamTransport::new(read, write)
            }
        } else {
            use super::*;
            use tokio::net::TcpStream;
            
            pub fn from_stream(stream: TcpStream) -> impl Transport {
                let (read, write) = stream.into_split();
                StreamTransport::new(read, write)
            }
            
            /// Same thing as [from_stream] if not on a `UNIX` based system
            pub fn from_tcp_stream(stream: TcpStream) -> impl Transport {
                from_stream(stream)
            }
        }
    }
}
