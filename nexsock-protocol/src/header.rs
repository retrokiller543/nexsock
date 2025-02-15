use crate::commands::Command;
use bincode::{Decode, Encode};
use binrw::{BinRead, BinResult, BinWrite};
#[cfg(feature = "savefile")]
use savefile::prelude::Savefile;
use std::io::{Read, Seek, Write};

#[cfg_attr(feature = "savefile", derive(Savefile))]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Encode, Decode)]
pub struct MessageFlags(u16);

impl MessageFlags {
    pub const NONE: MessageFlags = MessageFlags(0);
    pub const COMPRESSED: MessageFlags = MessageFlags(1 << 0);
    pub const ENCRYPTED: MessageFlags = MessageFlags(1 << 1);
    pub const REQUIRES_ACK: MessageFlags = MessageFlags(1 << 2);
    pub const HAS_PAYLOAD: MessageFlags = MessageFlags(1 << 3);

    pub fn contains(self, other: MessageFlags) -> bool {
        (self.0 & other.0) == other.0
    }

    pub fn is_empty(self) -> bool {
        self.0 == 0
    }
}

// Implement BitOr for combining flags
impl std::ops::BitOr for MessageFlags {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        MessageFlags(self.0 | rhs.0)
    }
}

// Implement BitAnd for checking flags
impl std::ops::BitAnd for MessageFlags {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        MessageFlags(self.0 & rhs.0)
    }
}

// Implement BinRead for MessageFlags
impl BinRead for MessageFlags {
    type Args<'a> = ();

    fn read_options<R: Read + Seek>(
        reader: &mut R,
        endian: binrw::Endian,
        _args: Self::Args<'_>,
    ) -> BinResult<Self> {
        let value = u16::read_options(reader, endian, ())?;
        Ok(MessageFlags(value))
    }
}

// Implement BinWrite for MessageFlags
impl BinWrite for MessageFlags {
    type Args<'a> = ();

    fn write_options<W: Write + Seek>(
        &self,
        writer: &mut W,
        endian: binrw::Endian,
        _args: Self::Args<'_>,
    ) -> BinResult<()> {
        self.0.write_options(writer, endian, ())
    }
}

#[cfg_attr(feature = "savefile", derive(Savefile))]
#[derive(Debug, BinRead, BinWrite, Encode, Decode)]
#[brw(magic = b"NEX\0", big)] // Magic bytes to identify our protocol
pub struct MessageHeader {
    #[brw(big)] // Explicitly set big endian
    pub(crate) version: u16,
    #[brw(big)] // Explicitly set big endian
    pub command: Command,
    #[brw(big)] // Explicitly set big endian
    pub(crate) payload_length: u32,
    #[brw(big)] // Explicitly set big endian
    pub(crate) sequence_number: u32,
    #[brw(big)] // Explicitly set big endian
    pub(crate) flags: MessageFlags,
}
