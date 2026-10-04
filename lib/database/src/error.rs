use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub enum DatabaseError {
    Storage(sled::Error),
    Serialization(serde_json::Error),
    Io(std::io::Error),
    NotFound,
    Conflict(String),
    InvalidInput(String),
}

impl DatabaseError {
    pub fn new(message: String) -> Self {
        Self::InvalidInput(message)
    }
}

impl Display for DatabaseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Storage(error) => write!(f, "Storage error: {error}"),
            Self::Serialization(error) => write!(f, "Invalid database record: {error}"),
            Self::Io(error) => write!(f, "Database asset error: {error}"),
            Self::NotFound => f.write_str("Record not found"),
            Self::Conflict(message) | Self::InvalidInput(message) => f.write_str(message),
        }
    }
}

impl std::error::Error for DatabaseError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Storage(error) => Some(error),
            Self::Serialization(error) => Some(error),
            Self::Io(error) => Some(error),
            _ => None,
        }
    }
}

impl From<sled::Error> for DatabaseError {
    fn from(error: sled::Error) -> Self {
        Self::Storage(error)
    }
}

impl From<serde_json::Error> for DatabaseError {
    fn from(error: serde_json::Error) -> Self {
        Self::Serialization(error)
    }
}

impl From<std::io::Error> for DatabaseError {
    fn from(error: std::io::Error) -> Self {
        Self::Io(error)
    }
}

impl From<sled::transaction::TransactionError<Self>> for DatabaseError {
    fn from(error: sled::transaction::TransactionError<Self>) -> Self {
        match error {
            sled::transaction::TransactionError::Abort(error) => error,
            sled::transaction::TransactionError::Storage(error) => Self::Storage(error),
        }
    }
}
