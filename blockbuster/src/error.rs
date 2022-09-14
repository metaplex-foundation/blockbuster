use std::io::Error;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum BlockbusterError {
    #[error("Instruction Data Parsing Error")]
    InstructionParsingError,
    #[error("IO Error {0}")]
    IOError(String),
    #[error("Could not deserialize data")]
    DeserializationError,
    #[error("Data length is invalid.")]
    InvalidDataLength,
}

impl From<std::io::Error> for BlockbusterError {
    fn from(err: Error) -> Self {
        BlockbusterError::IOError(err.to_string())
    }
}
