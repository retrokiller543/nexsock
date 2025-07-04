use crate::commands::Command;
use crate::header::{MessageFlags, MessageHeader};
use bincode::{Decode, Encode};
use binrw::{BinRead, BinWrite};
use std::fmt::Debug;
use std::io;
use tokio::io::{AsyncRead, AsyncReadExt, AsyncWrite, AsyncWriteExt};
use tracing::debug;
#[cfg(debug_assertions)]
use tracing::error;

#[derive(Debug, Default)]
pub struct Protocol {
    sequence: u32,
    version: u16,
}

impl Protocol {
    pub fn new(version: u16) -> Self {
        Self {
            sequence: 0,
            version,
        }
    }

    #[tracing::instrument(level = "debug", skip(writer))]
    pub async fn write_command<W>(&mut self, writer: &mut W, command: Command) -> io::Result<()>
    where
        W: AsyncWrite + Unpin,
    {
        self.write_message(writer, command, None::<&()>, MessageFlags::NONE)
            .await
    }

    #[tracing::instrument(level = "debug", skip(writer))]
    pub async fn write_command_with_payload<W, T: Encode + Debug>(
        &mut self,
        writer: &mut W,
        command: Command,
        payload: &T,
        flags: MessageFlags,
    ) -> io::Result<()>
    where
        W: AsyncWrite + Unpin,
    {
        let flags = flags | MessageFlags::HAS_PAYLOAD;
        self.write_message(writer, command, Some(payload), flags)
            .await
    }

    #[tracing::instrument(level = "debug", skip(writer))]
    async fn write_message<W, T: Encode + Debug>(
        &mut self,
        writer: &mut W,
        command: Command,
        payload: Option<&T>,
        flags: MessageFlags,
    ) -> io::Result<()>
    where
        W: AsyncWrite + Unpin,
    {
        // Serialize payload first to get length
        let payload_data = if let Some(payload) = payload {
            let config = bincode::config::standard();
            bincode::encode_to_vec(payload, config)
                .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?
        } else {
            Vec::new()
        };

        let header = MessageHeader {
            version: self.version,
            command,
            payload_length: payload_data.len() as u32,
            sequence_number: self.next_sequence(),
            flags,
        };

        // Create a cursor for writing the header
        let mut header_bytes = io::Cursor::new(Vec::new());
        header
            .write(&mut header_bytes)
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;

        // Write header
        writer.write_all(&header_bytes.into_inner()).await?;

        // Write payload if present
        if !payload_data.is_empty() {
            writer.write_all(&payload_data).await?;
        }

        writer.flush().await?;
        Ok(())
    }

    #[tracing::instrument(level = "debug", skip(reader))]
    /// Asynchronously reads a protocol message header and optional payload from the given reader.
    ///
    /// Validates the protocol magic bytes, deserializes the message header, and reads the payload if present.
    /// Returns the parsed message header and an optional payload as a byte vector.
    ///
    /// # Errors
    ///
    /// Returns an error if the magic bytes are invalid, if header or payload deserialization fails, or if I/O operations fail.
    ///
    /// # Examples
    ///
    /// ```
    /// # use your_crate::{Protocol, MessageHeader};
    /// # use tokio::io::BufReader;
    /// # async fn example() -> std::io::Result<()> {
    /// let mut protocol = Protocol::new(1);
    /// let data: &[u8] = /* some valid protocol message bytes */;
    /// let mut reader = BufReader::new(data);
    /// let (header, payload) = protocol.read_message(&mut reader).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn read_message<R>(
        &mut self,
        reader: &mut R,
    ) -> io::Result<(MessageHeader, Option<Vec<u8>>)>
    where
        R: AsyncRead + Unpin,
    {
        let mut magic = [0u8; 4];
        reader.read_exact(&mut magic).await?;

        if &magic != b"NEX\0" {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "Invalid protocol magic bytes",
            ));
        }

        let mut header_bytes = vec![0u8; size_of::<MessageHeader>() - 2]; // Subtract magic bytes
        reader.read_exact(&mut header_bytes).await?;

        let mut full_header = Vec::with_capacity(size_of::<MessageHeader>());
        full_header.extend_from_slice(&magic);
        full_header.extend_from_slice(&header_bytes);

        let header: MessageHeader = BinRead::read(&mut io::Cursor::new(full_header))
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;

        let payload = if header.flags.contains(MessageFlags::HAS_PAYLOAD) {
            let mut payload = vec![0u8; header.payload_length as usize];
            reader.read_exact(&mut payload).await?;

            Some(payload)
        } else {
            None
        };

        Ok((header, payload))
    }

    /// Decodes a payload byte slice into an optional value of type `T`.
    ///
    /// Returns `Ok(Some(data))` if the payload is non-empty and successfully decoded, `Ok(None)` if the payload is empty, or an error if decoding fails.
    ///
    /// # Type Parameters
    ///
    /// * `T` - The type to decode the payload into. Must implement `Decode<()>`.
    ///
    /// # Examples
    ///
    /// ```
    /// use your_crate::Protocol;
    /// use your_crate::YourType; // YourType must implement Decode<()>
    /// let payload: &[u8] = /* some encoded bytes */;
    /// let result: std::io::Result<Option<YourType>> = Protocol::read_payload(payload);
    /// ```
    pub fn read_payload<T: Decode<()>>(payload: &[u8]) -> io::Result<Option<T>> {
        let config = bincode::config::standard();

        if !payload.is_empty() {
            let (data, size) = match bincode::decode_from_slice(payload, config) {
                Ok(data) => data,
                Err(e) => {
                    #[cfg(debug_assertions)]
                    {
                        use crate::traits::PayloadDebug;
                        error!("{}", payload.debug_dump());
                    }
                    return Err(io::Error::new(io::ErrorKind::InvalidData, e));
                }
            };

            debug!("Read payload of size: `{size}`");

            Ok(Some(data))
        } else {
            Ok(None)
        }
    }

    fn next_sequence(&mut self) -> u32 {
        let seq = self.sequence;
        self.sequence = self.sequence.wrapping_add(1);
        seq
    }
}
