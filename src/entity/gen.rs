use serde::{Deserialize, Serialize};

use std::path::PathBuf;

/// Configuration for a generator.
#[derive(Serialize, Deserialize, Clone, PartialEq, PartialOrd, Debug)]
pub struct GenConfig {
    pub language_name: String,
    pub path: PathBuf,
}
