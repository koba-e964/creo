use serde::{Deserialize, Serialize};

/// Configuration for solution files.
#[derive(Serialize, Deserialize, Clone, PartialEq, PartialOrd, Debug)]
pub struct SolutionConfig {
    /// Path to the solution file.
    pub path: String,
    /// In which language is this solution written?
    pub language_name: String,
    /// What kind of verdict should this solution receive?
    #[serde(default)]
    #[serde(skip_serializing_if = "is_ac")]
    pub expected_verdict: Verdict,
    #[serde(default)]
    #[serde(skip_serializing_if = "is_false")]
    pub is_reference_solution: bool,
}

/// Judge's verdict. Bigger it is, worse it is.
#[derive(Serialize, Deserialize, Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Default)]
#[serde(rename_all = "lowercase")]
pub enum Verdict {
    /// ACcepted. The solution is correct.
    #[default]
    AC,
    /// Time Limit Exceeded. The solution didn't finish before the predetermined time limit.
    TLE,
    /// Wrong Answer. The solution's output didn't match judge's output.
    WA,
    /// Runtime Error.
    RE,
    /// Memory Limit Exceeded. The solution used memory more than the limit.
    MLE,
    /// Query Limit Exceeded. The solution issued queries more than predetermined query limit.
    QLE,
    /// Internal Error. It is the fault of the judge, not of the solution.
    IE,
}

fn is_ac(x: &Verdict) -> bool {
    x == &Verdict::AC
}

fn is_false(x: &bool) -> bool {
    !x
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn verdict_serialize_work() {
        let ser = toml::to_string(&Verdict::AC).unwrap();
        assert_eq!(ser, "\"ac\"");
        let ser = toml::to_string(&Verdict::WA).unwrap();
        assert_eq!(ser, "\"wa\"");
    }

    #[test]
    fn verdict_deserialize_work() {
        let de: Verdict = toml::from_str("\"ac\"").unwrap();
        assert_eq!(de, Verdict::AC);
        let de: Verdict = toml::from_str("\"wa\"").unwrap();
        assert_eq!(de, Verdict::WA);
    }

    #[test]
    fn solution_serialize_work() {
        let ser = toml::to_string(&SolutionConfig {
            path: "test".to_owned(),
            language_name: "C++".to_owned(),
            expected_verdict: Verdict::AC,
            is_reference_solution: false,
        })
        .unwrap();
        // expected_verdict is skipped because it is AC.
        // is_reference_solution is skipped because it is false.
        let expected = r#"path = "test"
language_name = "C++"
"#;
        assert_eq!(ser, expected);
        let ser = toml::to_string(&SolutionConfig {
            path: "test".to_owned(),
            language_name: "Rust".to_owned(),
            expected_verdict: Verdict::WA,
            is_reference_solution: true,
        })
        .unwrap();
        // expected_verdict is serialized because it is not AC.
        // is_reference_solution is serialized because it is true.
        let expected = r#"path = "test"
language_name = "Rust"
expected_verdict = "wa"
is_reference_solution = true
"#;
        assert_eq!(ser, expected);
    }
}
