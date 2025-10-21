use thiserror::Error;

#[derive(Error, Debug)]
pub enum ChirpError {
    #[error("Data requires buffer of {required} length, but only {available} available.")]
    Buffer { required: usize, available: usize },
}
