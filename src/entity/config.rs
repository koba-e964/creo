use serde::{Deserialize, Serialize};

use super::gen::GenConfig;

/// Config file for creo.
/// Should be placed at creo.toml
#[derive(Serialize, Deserialize, Clone, PartialEq, PartialOrd, Debug)]
pub struct CreoConfig {
    #[serde(default = "time_limit_default")]
    time_limit: f64,
    #[serde(default)]
    generators: Vec<GenConfig>,
    #[serde(default)]
    languages: Vec<LanguageConfig>,
}

fn time_limit_default() -> f64 {
    2.0
}

/// Configuration for an available language.
#[derive(Serialize, Deserialize, Clone, PartialEq, PartialOrd, Debug)]
pub struct LanguageConfig {
    /// name
    language_name: String,
    /// target extension
    target_ext: String,
    /// How can we compile the source code?
    compile: String,
    /// How can we run the compiled binary?
    /// If the given code is a script, this should run the original script.
    run: String,
}

impl Default for CreoConfig {
    fn default() -> Self {
        let cpp = LanguageConfig {
            language_name: "C++".to_owned(),
            target_ext: ".cpp".to_owned(),
            compile: "g++ -O2 -std=gnu++11 -o a.out".to_owned(),
            run: "./a.out".to_owned(),
        };
        let python = LanguageConfig {
            language_name: "Python".to_owned(),
            target_ext: ".py".to_owned(),
            compile: "true".to_owned(),
            run: "python3".to_owned(),
        };
        Self {
            time_limit: 2.0,
            generators: vec![],
            languages: vec![cpp, python],
        }
    }
}
