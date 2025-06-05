use crate::commands::manage_service::ServiceRef;
use crate::commands::CommandPayload;
use crate::{service_command, try_from};
use bincode::{Decode, Encode};
use derive_more::Display;
#[cfg(feature = "savefile")]
use savefile::prelude::Savefile;

cfg_if::cfg_if! {
    if #[cfg(feature = "sea-orm")] {
        use sea_orm::sea_query::{ArrayType, ValueType, ValueTypeErr};
        use sea_orm::{ColIdx, ColumnType, DbErr, QueryResult, TryGetError, TryGetable, Value};
        use sea_orm::prelude::StringLen;
    }
}

use serde::{Deserialize, Serialize};
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
    Display,
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
    /// Converts an optional string into a `ConfigFormat`, defaulting to `Env` if the input is `None` or unrecognized.
    ///
    /// # Examples
    ///
    /// ```
    /// use your_crate::ConfigFormat;
    /// assert_eq!(ConfigFormat::from(Some("env".to_string())), ConfigFormat::Env);
    /// assert_eq!(ConfigFormat::from(Some("properties".to_string())), ConfigFormat::Properties);
    /// assert_eq!(ConfigFormat::from(None), ConfigFormat::Env);
    /// ```
    fn from(value: Option<String>) -> Self {
        if let Some(val) = value {
            val.into()
        } else {
            Self::Env
        }
    }
}

#[cfg(feature = "sea-orm")]
impl ValueType for ConfigFormat {
    /// Attempts to convert a `Value` into a `ConfigFormat` enum variant.
    ///
    /// Returns `Ok(ConfigFormat)` if the input is a string matching "Env" or "Properties"; otherwise, returns `Err(ValueTypeErr)`.
    fn try_from(v: Value) -> Result<Self, ValueTypeErr> {
        match v {
            Value::String(Some(x)) => match x.as_str() {
                "Env" => Ok(Self::Env),
                "Properties" => Ok(Self::Properties),
                _ => Err(ValueTypeErr),
            },
            _ => Err(ValueTypeErr),
        }
    }

    /// Returns the type name for the `ConfigFormat` enum as a string.
    ///
    /// # Examples
    ///
    /// ```
    /// let name = type_name();
    /// assert_eq!(name, "ConfigFormat");
    /// ```
    fn type_name() -> String {
        String::from("ConfigFormat")
    }

    /// This is used by SeaORM to define the column type for arrays of `ConfigFormat` values.
    fn array_type() -> ArrayType {
        ArrayType::String
    }

    /// Returns the database column type for `ConfigFormat` as an unconstrained string.
    ///
    /// # Examples
    ///
    /// ```
    /// use sea_orm::ColumnType;
    /// assert_eq!(column_type(), ColumnType::String(sea_orm::StringLen::None));
    /// ```
    fn column_type() -> ColumnType {
        ColumnType::String(StringLen::None)
    }
}

#[cfg(feature = "sea-orm")]
impl From<ConfigFormat> for Value {
    /// Converts a `ConfigFormat` value into a `sea_orm::Value::String` containing its string representation.
    fn from(config_format: ConfigFormat) -> Self {
        Value::String(Some(Box::new(config_format.to_string())))
    }
}

#[cfg(feature = "sea-orm")]
impl TryGetable for ConfigFormat {
    /// Attempts to extract a `ConfigFormat` value from a database query result at the specified column index.
    ///
    /// Returns an error if the value is not a recognized config format string ("Env" or "Properties").
    fn try_get_by<I: ColIdx>(res: &QueryResult, index: I) -> Result<Self, TryGetError> {
        let val: String = res.try_get_by(index)?;

        match val.as_str() {
            "Env" => Ok(Self::Env),
            "Properties" => Ok(Self::Properties),
            val => Err(TryGetError::DbErr(DbErr::Custom(format!(
                "`{val}` is not a valid config format"
            )))),
        }
    }
}
