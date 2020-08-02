use serde::{Deserialize, Serialize};

/// Configuration for solution files.
#[derive(Serialize, Deserialize, Clone, PartialEq, PartialOrd, Debug)]
pub struct SolutionConfig {
    /// Path to the solution file.
    pub path: String,
    /// In which language is theis solution written?
    pub language_name: String,
    /// What kind of verdict should this solution receive?
    pub expected_verdict: Verdict,
}

/// Judge's verdict.
#[derive(Serialize, Deserialize, Clone, PartialEq, PartialOrd, Debug)]
#[serde(rename_all = "lowercase")]
pub enum Verdict {
    /// ACcepted. The solution is correct.
    AC,
    /// Wrong Answer. The solution's output didn't match judge's output.
    WA,
    /// Time Limit Exceeded. The solution didn't finish before the predetermined time limit.
    TLE,
    /// Memory Limit Exceeded. The solution used memory more than the limit.
    MLE,
    /// Query Limit Exceeded. The solution issued queries more than predetermined query limit.
    QLE,
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
}
