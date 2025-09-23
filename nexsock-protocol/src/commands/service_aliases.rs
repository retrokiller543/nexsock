use crate::commands::manage_service::ServiceRef;
use crate::service_command;
use bincode::{Decode, Encode};
#[cfg(feature = "savefile")]
use savefile::prelude::Savefile;
use serde::{Deserialize, Serialize};

// List aliases
#[cfg_attr(feature = "savefile", derive(savefile::prelude::Savefile))]
#[derive(Clone, Default, Debug, PartialEq, Eq, Serialize, Deserialize, Encode, Decode)]
pub struct ListServiceAliasesPayload {
    pub service: ServiceRef,
}

service_command! {
    pub struct ListServiceAliases<ListServiceAliasesPayload, AliasesResponse> = ListServiceAliases {
        service: ServiceRef
    }
}

#[cfg_attr(feature = "savefile", derive(savefile::prelude::Savefile))]
#[derive(Clone, Default, Debug, PartialEq, Eq, Serialize, Deserialize, Encode, Decode)]
pub struct AliasesResponse {
    pub service_name: String,
    pub aliases: Vec<String>,
}

// Add aliases
#[cfg_attr(feature = "savefile", derive(savefile::prelude::Savefile))]
#[derive(Clone, Default, Debug, PartialEq, Eq, Serialize, Deserialize, Encode, Decode)]
pub struct AddServiceAliasesPayload {
    pub service: ServiceRef,
    pub aliases: Vec<String>,
}

service_command! {
    pub struct AddServiceAliases<AddServiceAliasesPayload, ()> = AddServiceAliases {
        service: ServiceRef,
        aliases: Vec<String>
    }
}

// Remove aliases
#[cfg_attr(feature = "savefile", derive(savefile::prelude::Savefile))]
#[derive(Clone, Default, Debug, PartialEq, Eq, Serialize, Deserialize, Encode, Decode)]
pub struct RemoveServiceAliasesPayload {
    pub service: ServiceRef,
    pub aliases: Vec<String>,
}

service_command! {
    pub struct RemoveServiceAliases<RemoveServiceAliasesPayload, ()> = RemoveServiceAliases {
        service: ServiceRef,
        aliases: Vec<String>
    }
}

// Resolve service alias
#[cfg_attr(feature = "savefile", derive(savefile::prelude::Savefile))]
#[derive(Clone, Default, Debug, PartialEq, Eq, Serialize, Deserialize, Encode, Decode)]
pub struct ResolveServiceAliasPayload {
    pub alias: String,
}

service_command! {
    pub struct ResolveServiceAlias<ResolveServiceAliasPayload, ResolveAliasResponse> = ResolveServiceAlias {
        alias: String
    }
}

#[cfg_attr(feature = "savefile", derive(savefile::prelude::Savefile))]
#[derive(Clone, Default, Debug, PartialEq, Eq, Serialize, Deserialize, Encode, Decode)]
pub struct ResolveAliasResponse {
    pub alias: String,
    pub service_id: Option<i64>,
    pub service_name: Option<String>,
}