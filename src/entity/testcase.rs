use serde::{Deserialize, Serialize};

/// Configuration for input/output files.
#[derive(Serialize, Deserialize, Clone, PartialEq, PartialOrd, Debug)]
pub struct TestcaseConfig {
    /// Where input files are located. Before generation, files in this directory will be DELETED.
    #[serde(default = "indir_default")]
    pub indir: String,
    /// Where output files are located. Before generation, files in this directory will be DELETED.
    #[serde(default = "outdir_default")]
    pub outdir: String,
}

fn indir_default() -> String {
    "in".to_owned()
}

fn outdir_default() -> String {
    "out".to_owned()
}

impl Default for TestcaseConfig {
    fn default() -> Self {
        Self {
            indir: indir_default(),
            outdir: outdir_default(),
        }
    }
}
