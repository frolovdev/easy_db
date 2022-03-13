use serde::{Deserialize, Serialize};

/// A datatype
#[derive(Clone, Debug, Hash, PartialEq, Serialize, Deserialize)]
pub enum DataType {
    Boolean,
    Integer,
    Float,
    String,
}
