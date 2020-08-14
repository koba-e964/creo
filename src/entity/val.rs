use serde::{Deserialize, Serialize};

/// Configuration for validator files.
#[derive(Serialize, Deserialize, Clone, PartialEq, PartialOrd, Debug)]
pub struct ValidatorConfig {
    /// Path to the validator file.
    pub path: String,
    /// In which language is this validator written?
    pub language_name: String,
}
