use bincode::{config, Decode, Encode};
use bytes::Bytes;
use crate::error::ProtocolError;
use crate::frame::{Frame, FrameFlags};

/// Trait for messages that can be sent over the protocol
pub trait Message: Sized {
    const MESSAGE_TYPE_ID: u16;

    fn serialize(&self) -> Result<Bytes, ProtocolError>;

    /// Deserialize a message from bytes
    fn deserialize(bytes: Bytes) -> Result<Self, ProtocolError>;

    /// Convert the message to a frame
    fn to_frame(&self, sequence: u32) -> Result<Frame, ProtocolError> {
        let payload = self.serialize()?;
        let has_payload = !payload.is_empty();
        let flags = if has_payload { FrameFlags::HAS_PAYLOAD } else { FrameFlags::NONE };

        Ok(Frame::new(
            Self::MESSAGE_TYPE_ID,
            sequence,
            flags,
            payload,
        ))
    }

    /// Create a message from a frame
    fn from_frame(frame: Frame) -> Result<Self, ProtocolError> {
        if frame.has_payload() {
            Self::deserialize(frame.payload)
        } else {
            Err(ProtocolError::ExpectedPayload)
        }
    }
}

/*/// Helper trait for messages with binary serialization
pub trait BinaryMessage: Message {
    /// Serialize the message to bytes
    fn serialize(&self) -> Result<Bytes, ProtocolError>;

    /// Deserialize a message from bytes
    fn deserialize(bytes: Bytes) -> Result<Self, ProtocolError>;

    /// Default implementation of to_frame for binary messages
    fn default_to_frame(&self, sequence: u32) -> Result<Frame, ProtocolError> {
        let payload = self.serialize()?;
        let has_payload = !payload.is_empty();
        let flags = if has_payload { FrameFlags::HAS_PAYLOAD } else { FrameFlags::NONE };

        Ok(Frame::new(
            Self::message_type(),
            sequence,
            flags,
            payload,
        ))
    }

    /// Default implementation of from_frame for binary messages
    fn default_from_frame(frame: Frame) -> Result<Self, ProtocolError> {
        if frame.has_payload() {
            Self::deserialize(frame.payload)
        } else {
            Err(ProtocolError::ExpectedPayload)
        }
    }
}*/

/// Helper trait for messages using bincode serialization
pub trait BincodeMessage: Encode + Decode<()> {
    /// Default serialization using bincode
    fn bincode_serialize(&self) -> Result<Bytes, ProtocolError> {
        let config = config::standard();
        let result = bincode::encode_to_vec(self, config)
            .map_err(|e| ProtocolError::Serialization {
                error: Box::new(e),
            })?;

        Ok(Bytes::from(result))
    }

    /// Default deserialization using bincode
    fn bincode_deserialize(bytes: Bytes) -> Result<Self, ProtocolError> {
        let config = config::standard();
        let (result, _) = bincode::decode_from_slice(bytes.as_ref(), config)
            .map_err(|e| ProtocolError::Deserialization { error: Box::new(e) })?;

        Ok(result)
    }
}

impl<T: Encode + Decode<()>> BincodeMessage for T {}

/*/// Implement BinaryMessage for any type that implements BincodeMessage
impl<T: BincodeMessage> BinaryMessage for T {
    fn serialize(&self) -> Result<Bytes, ProtocolError> {
        self.bincode_serialize()
    }

    fn deserialize(bytes: Bytes) -> Result<Self, ProtocolError> {
        Self::bincode_deserialize(bytes)
    }
}
*/