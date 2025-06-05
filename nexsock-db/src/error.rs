#[derive(Debug, thiserror::Error)]
pub enum DatabaseError {
    #[error("Unknown enum value `{value}` for enum `{enum_name}`. Expected one of: {expected}")]
    UnknownEnumValue {
        enum_name: String,
        value: String,
        expected: String,
    },
    #[error("Unsupported database: {database}")]
    UnsupportedDatabase { database: String },
    #[error("Database URL is empty")]
    EmptyDatabaseUrl,
    #[error("Failed to parse database URL: {0}")]
    InvalidDatabaseUrl(#[from] url::ParseError),
    #[error("Failed to decode URL path: {0}")]
    PathDecoding(#[from] std::str::Utf8Error),
    #[error("Invalid SQLite path: {0}")]
    InvalidSqlitePath(String),
    #[error("SQLite path is a directory: {0}")]
    SqlitePathIsDir(String),
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),
}
