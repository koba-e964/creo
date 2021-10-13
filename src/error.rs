use crate::entity::sol::Verdict;

#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub enum Error {
    #[error("I/O error occurred")]
    IOError(
        #[from]
        #[source]
        std::io::Error,
    ),
    #[error("Config file has invalid configuration: {description}")]
    ConfInvalid { description: String },
    #[error("Verdict is not as expected (expected = {expected:?}, actual = {actual:?})")]
    VerdictMismatch { expected: Verdict, actual: Verdict },
    #[error("Toml serialization failed")]
    TomlSerError(
        #[from]
        #[source]
        toml::ser::Error,
    ),
    #[error("Toml deserialization failed")]
    TomlDeError(
        #[from]
        #[source]
        toml::de::Error,
    ),
    #[error("Unknown entity type: {entity_type}")]
    UnknownEntityType { entity_type: String },
    #[error("Validation failed: validator = {validator}, infile = {infile}")]
    ValidationFailed {
        validator: String,
        infile: String,
        inner: Box<dyn std::error::Error + 'static>,
    },
}

pub type Result<T> = std::result::Result<T, Error>;
