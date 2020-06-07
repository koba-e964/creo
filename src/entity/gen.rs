use serde::{Deserialize, Serialize};

use std::path::PathBuf;

/// Configuration for a generator.
#[derive(Serialize, Deserialize, Clone, PartialEq, PartialOrd, Debug)]
pub struct GenConfig {
    language_name: String,
    path: PathBuf,
}
