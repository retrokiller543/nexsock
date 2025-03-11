use std::convert::Infallible;
use crate::commands::manage_service::ServiceRef;
use crate::commands::CommandPayload;
use crate::{service_command, try_from};
use bincode::{Decode, Encode};
use derive_more::Display;
#[cfg(feature = "savefile")]
use savefile::prelude::Savefile;
use sea_orm::sea_query::{ArrayType, ValueType, ValueTypeErr};
use sea_orm::{ColIdx, ColumnType, DbErr, QueryResult, TryGetError, TryGetable, Value};
use sea_orm::prelude::StringLen;
use serde::{Deserialize, Serialize};
use serde::__private::de::IdentifierDeserializer;
use sqlx::Type;

service_command! {
    pub struct GetConfig<ServiceRef, ServiceConfigPayload> = GetConfig
}

service_command! {
    pub struct UpdateConfigCommand<ServiceConfigPayload, ()> = UpdateConfig {
        service: ServiceRef,
        filename: String,
        format: ConfigFormat,
        run_command: String
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
pub struct ServiceConfigPayload {
    pub service: ServiceRef,
    pub filename: String,
    pub format: ConfigFormat,
    pub run_command: String,
}

try_from!(ServiceConfig => ServiceConfigPayload);

#[cfg_attr(feature = "savefile", derive(Savefile))]
#[derive(
    Clone,
    Copy,
    Default,
    Debug,
    Ord,
    PartialOrd,
    PartialEq,
    Eq,
    Hash,
    Serialize,
    Deserialize,
    Type,
    Encode,
    Decode,
    Display
)]
#[serde(rename_all = "lowercase")]
#[repr(u8)]
pub enum ConfigFormat {
    #[default]
    Env,
    Properties,
}

impl From<String> for ConfigFormat {
    fn from(value: String) -> Self {
        match value.as_str() {
            "Env" => Self::Env,
            "Properties" => Self::Properties,
            _ => Self::Env,
        }
    }
}

impl From<Option<String>> for ConfigFormat {
    fn from(value: Option<String>) -> Self {
        if let Some(val) = value {
            val.into()
        } else {
            Self::Env
        }
    }
}

impl ValueType for ConfigFormat {
    fn try_from(v: Value) -> Result<Self, ValueTypeErr> {
        match v {
            Value::String(Some(x)) => {
                match x.as_str() {
                    "Env" => Ok(Self::Env),
                    "Properties" => Ok(Self::Properties),
                    _ => Err(ValueTypeErr),
                }
            }
            _ => Err(ValueTypeErr),
        }
    }

    fn type_name() -> String {
        String::from("ConfigFormat")
    }

    fn array_type() -> ArrayType {
        ArrayType::String
    }

    fn column_type() -> ColumnType {
        ColumnType::String(StringLen::None)
    }
}

impl From<ConfigFormat> for Value {
    fn from(config_format: ConfigFormat) -> Self {
        Value::String(Some(Box::new(config_format.to_string())))
    }
}

impl TryGetable for ConfigFormat {
    fn try_get_by<I: ColIdx>(res: &QueryResult, index: I) -> Result<Self, TryGetError> {
        let val: String = res.try_get_by(index)?;

        match val.as_str() {
            "Env" => Ok(Self::Env),
            "Properties" => Ok(Self::Properties),
            val => Err(TryGetError::DbErr(DbErr::Custom(format!("`{val}` is not a valid config format")))),
        }
    }
}