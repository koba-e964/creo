use crate::entity::sol::Verdict;

#[derive(Debug, thiserror::Error)]
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
}

pub type Result<T> = std::result::Result<T, Error>;
