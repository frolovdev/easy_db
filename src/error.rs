use serde::{Deserialize, Serialize};
use std::fmt::{self, Display};

/// Internal is internal error
/// everything else is users
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum EasyDbError {
    Internal(String),
    Parse(String),
    Value(String),
}

/// Result returning Error
pub type EasyDbResult<T> = std::result::Result<T, EasyDbError>;
