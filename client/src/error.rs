#[derive(thiserror::Error, Debug)]
pub enum CommunicationError {
    #[error("Generic {0}")]
    Generic(String),
}

pub type Result<T> = core::result::Result<T, CommunicationError>;
