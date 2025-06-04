use thiserror::Error;

pub type ModemResult<T> = Result<T, ModemError>;

#[derive(Error, Debug)]
pub enum ModemError {
    #[error("Failed to create modem.")]
    CreationError,

    #[error("Failed to destroy modem: {0}")]
    DestructionError(String),

    #[error("Invalid parameter: {0}")]
    InvalidParameter(String),

    #[error("Operation failed: {0}")]
    OperationFailed(String),
}