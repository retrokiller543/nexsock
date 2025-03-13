use bytes::{Buf, BufMut, Bytes, BytesMut};
use std::io;
use derive_more::{AsMut, AsRef, BitAnd, BitAndAssign, BitOr, BitOrAssign, Deref, DerefMut};

/// Protocol frame flags
#[derive(Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd, BitOr, BitAnd, BitAndAssign, BitOrAssign, Deref, DerefMut, AsRef, AsMut)]
pub struct FrameFlags(pub u16);

impl FrameFlags {
    // Common flags - can be extended as needed
    pub const NONE: Self = Self(0);
    pub const HAS_PAYLOAD: Self = Self(1 << 0);
    pub const COMPRESSED: Self = Self(1 << 1);
    pub const ENCRYPTED: Self = Self(1 << 2);
    pub const REQUIRES_ACK: Self = Self(1 << 3);

    pub fn contains(self, other: Self) -> bool {
        (self.0 & other.0) == other.0
    }

    pub fn is_empty(self) -> bool {
        self.0 == 0
    }

    pub fn as_u16(self) -> u16 {
        self.0
    }

    pub fn from_u16(value: u16) -> Self {
        Self(value)
    }
}

/// Protocol frame structure - this is the wire format
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Frame {
    /// Magic bytes for validation (always "NEX\0")
    pub magic: [u8; 4],

    /// Protocol version
    pub version: u16,

    /// Message type identifier
    pub message_type: u16,

    /// Sequence number for message ordering
    pub sequence: u32,

    /// Frame flags
    pub flags: FrameFlags,

    /// Message payload
    pub payload: Bytes,
}

impl Frame {
    /// The magic bytes used to identify this protocol
    pub const MAGIC: [u8; 4] = *b"NEX\0";

    /// Default protocol version
    pub const DEFAULT_VERSION: u16 = 1;

    /// Create a new frame
    pub fn new(message_type: u16, sequence: u32, flags: FrameFlags, payload: Bytes) -> Self {
        Self {
            magic: Self::MAGIC,
            version: Self::DEFAULT_VERSION,
            message_type,
            sequence,
            flags,
            payload,
        }
    }

    /// Check if the frame has a payload
    pub fn has_payload(&self) -> bool {
        self.flags.contains(FrameFlags::HAS_PAYLOAD)
    }

    /// Get the header size
    pub const fn header_size() -> usize {
        4 + // magic
        2 + // version
        2 + // message_type
        4 + // sequence
        2   // flags
    }

    /// Encode the frame to bytes
    pub fn encode(&self) -> io::Result<BytesMut> {
        let payload_len = self.payload.len();
        let total_len = Self::header_size() + 4 + payload_len; // +4 for payload length

        let mut buf = BytesMut::with_capacity(total_len);

        // Write header
        buf.put_slice(&self.magic);
        buf.put_u16(self.version);
        buf.put_u16(self.message_type);
        buf.put_u32(self.sequence);
        buf.put_u16(self.flags.as_u16());

        // Write payload length and payload
        buf.put_u32(payload_len as u32);
        if !self.payload.is_empty() {
            buf.put_slice(&self.payload);
        }

        Ok(buf)
    }

    /// Decode a frame from bytes
    pub fn decode(mut buf: Bytes) -> io::Result<Self> {
        if buf.len() < Self::header_size() + 4 {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "Buffer too small for frame header",
            ));
        }

        // Read magic bytes
        let mut magic = [0u8; 4];
        buf.copy_to_slice(&mut magic);

        // Validate magic bytes
        if magic != Self::MAGIC {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "Invalid magic bytes",
            ));
        }

        // Read header fields
        let version = buf.get_u16();
        let message_type = buf.get_u16();
        let sequence = buf.get_u32();
        let flags = FrameFlags::from_u16(buf.get_u16());

        // Read payload length and payload
        let payload_len = buf.get_u32() as usize;
        if buf.len() < payload_len {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "Buffer too small for payload",
            ));
        }

        let payload = if payload_len > 0 {
            let payload_buf = buf.slice(0..payload_len);
            buf.advance(payload_len);
            payload_buf
        } else {
            Bytes::new()
        };

        Ok(Self {
            magic,
            version,
            message_type,
            sequence,
            flags,
            payload,
        })
    }
}