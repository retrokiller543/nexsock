use crate::commands::service_status::ServiceState;
use crate::commands::CommandPayload;
use crate::{service_command, try_from};
use bincode::{Decode, Encode};
use bytes::Bytes;
use derive_more::AsRef;
#[cfg(feature = "savefile")]
use savefile::prelude::Savefile;
use serde::{Deserialize, Serialize};
use nexsock_protocol_core::error::ProtocolError;
use nexsock_protocol_core::frame::Frame;
use nexsock_protocol_core::prelude::{BincodeMessage, Message};

#[cfg_attr(feature = "savefile", derive(Savefile))]
#[derive(
    Clone,
    Default,
    Debug,
    Ord,
    PartialOrd,
    PartialEq,
    Eq,
    Hash,
    Serialize,
    Deserialize,
    Encode,
    Decode,
    AsRef,
)]
pub struct ListServicesResponse {
    pub services: Vec<ServiceInfo>,
}

impl Message for ListServicesResponse {
    const MESSAGE_TYPE_ID: u16 = 1;

    fn serialize(&self) -> Result<bytes::Bytes, ProtocolError> {
        self.bincode_serialize()
    }

    fn deserialize(bytes: bytes::Bytes) -> Result<Self, ProtocolError> {
        Self::bincode_deserialize(bytes)
    }
}

service_command! {
    pub struct ListServicesCommand<_, ListServicesResponse> = ListServices
}

impl Message for ListServicesCommand {
    const MESSAGE_TYPE_ID: u16 = 0;

    fn serialize(&self) -> Result<Bytes, ProtocolError> {
        Ok(Bytes::new())
    }

    fn deserialize(bytes: Bytes) -> Result<Self, ProtocolError> {
        Ok(Self)
    }
}

try_from!(ListServices => ListServicesResponse);

impl<T: Into<ServiceInfo>> FromIterator<T> for ListServicesResponse {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        ListServicesResponse {
            services: iter.into_iter().map(Into::into).collect(),
        }
    }
}

#[cfg_attr(feature = "savefile", derive(Savefile))]
#[derive(
    Clone,
    Default,
    Debug,
    Ord,
    PartialOrd,
    PartialEq,
    Eq,
    Hash,
    Serialize,
    Deserialize,
    Encode,
    Decode,
)]
pub struct ServiceInfo {
    pub name: String,
    pub state: ServiceState,
    pub port: i64,
    pub has_dependencies: bool,
}
