use serde::{Deserialize, Serialize};

use super::gen::GenConfig;
use super::sol::SolutionConfig;
use super::testcase::TestcaseConfig;
use super::val::ValidatorConfig;

/// Config file for creo.
/// Should be placed at creo.toml
#[derive(Serialize, Deserialize, Clone, PartialEq, PartialOrd, Debug)]
pub struct CreoConfig {
    /// Time limit in seconds.
    #[serde(default = "time_limit_default")]
    pub time_limit: f64,
    /// Generators.
    #[serde(default)]
    // Needed by toml: https://github.com/alexcrichton/toml-rs/issues/258.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub generators: Vec<GenConfig>,
    /// Available languages.
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub languages: Vec<LanguageConfig>,
    /// Solutions.
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub solutions: Vec<SolutionConfig>,
    /// Validators.
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub validators: Vec<ValidatorConfig>,
    /// Configuration for input/output files.
    #[serde(default)]
    pub testcase_config: TestcaseConfig,
}

fn time_limit_default() -> f64 {
    2.0
}

/// Configuration for an available language.
#[derive(Serialize, Deserialize, Clone, PartialEq, PartialOrd, Debug)]
pub struct LanguageConfig {
    /// name
    pub language_name: String,
    /// target extension, e.g. "cpp", "py"
    pub target_ext: String,
    /// How can we compile the source code?
    pub compile: Vec<String>,
    /// How can we run the compiled binary?
    /// If the given code is a script, this should run the original script.
    pub run: Vec<String>,
}

impl Default for CreoConfig {
    fn default() -> Self {
        let cpp = LanguageConfig {
            language_name: "C++".to_owned(),
            target_ext: "cpp".to_owned(),
            compile: vec!["g++", "-O2", "-std=gnu++11", "-o", "$OUT", "$IN"]
                .into_iter()
                .map(|x| x.to_owned())
                .collect(),
            run: vec!["$OUT".to_owned()],
        };
        let python = LanguageConfig {
            language_name: "Python".to_owned(),
            target_ext: "py".to_owned(),
            compile: vec!["cp", "$IN", "$OUT"]
                .into_iter()
                .map(|x| x.to_owned())
                .collect(),
            run: vec!["python3".to_owned(), "$OUT".to_owned()],
        };
        let testcase_config = TestcaseConfig {
            indir: "in".to_owned(),
            outdir: "out".to_owned(),
        };
        Self {
            time_limit: 2.0,
            generators: vec![],
            languages: vec![cpp, python],
            solutions: vec![],
            validators: vec![],
            testcase_config,
        }
    }
}
