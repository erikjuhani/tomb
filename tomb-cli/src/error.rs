use thiserror::Error;

#[derive(Debug, Error)]
pub enum TombError {
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),
}

pub type Result<T> = std::result::Result<T, TombError>;
