use thiserror::Error;

#[derive(Debug, Error)]
pub enum TombError {
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Config error: {0}")]
    Config(String),
    #[error("Parse error: {0}")]
    Parse(String),
    #[error("Unsupported version: {0}")]
    UnsupportedVersion(String),
    #[error("Id not found: ${0}")]
    IdNotFound(String),
    #[error("Ambiguous id {prefix}: matches {}", .matches.join(", "))]
    AmbiguousId { prefix: String, matches: Vec<String> },
}

pub type Result<T> = std::result::Result<T, TombError>;
