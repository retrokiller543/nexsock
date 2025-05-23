#[derive(Debug, thiserror::Error)]
pub enum DatabaseError {
    #[error("Unknown enum value `{value}` for enum `{enum_name}`. Expected one of: {expected}")]
    UnknownEnumValue {
        enum_name: String,
        value: String,
        expected: String,
    },
}