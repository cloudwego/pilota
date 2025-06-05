#[derive(Debug, thiserror::Error)]
pub enum ReflectorError {
    #[error("Serialization error: {0}")]
    SerializationError(String),
    #[error("Deserialization error: {0}")]
    DeserializationError(String),
    #[error("Include path error: {0}")]
    IncludePathError(String),
}
