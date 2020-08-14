use path_clean::PathClean;
use std::path::Path;

use crate::entity::config::CreoConfig;
use crate::entity::sol::Verdict;
use crate::entity::testcase::TestcaseConfig;
use crate::error::{Error, Result};
use crate::io_util::{IoUtil, IoUtilExt};
use crate::run_util::{RunUtil, RunUtilExt};
/// A trait that provides functions to handle a project directory.
pub trait Project {
    /// Check if the config file is valid.
    #[allow(unused)]
    fn check(&mut self, proj_dir: &str) -> Result<()> {
        unreachable!();
    }
    /// Generate input files from a generator.
    #[allow(unused)]
    fn gen(&mut self, proj_dir: &str) -> Result<()> {
        unreachable!();
    }
    /// Generate output files from input files and a reference solution.
    #[allow(unused)]
    fn refgen(&mut self, proj_dir: &str) -> Result<()> {
        unreachable!();
    }
    /// Execute all solutions and check if their output matches expected output.
    #[allow(unused)]
    fn test(&mut self, proj_dir: &str) -> Result<()> {
        unreachable!();
    }
}

pub trait ProjectExt: IoUtil + RunUtil {
    fn read_config(&mut self, proj: &Path) -> Result<CreoConfig> {
        // Read the config file
        let config_filepath = proj.join("creo.toml");
        let mut file = self.open_file_for_read(&config_filepath)?;
        let content = self.read_from_file(&mut file)?;
        // TODO: better error handling (user-defined error type probably helps)
        let config: CreoConfig = toml::from_str(&content)?;

        Ok(config)
    }

    // Given the result of execution and the expected output file, find the verdict.
    fn get_verdict(&mut self, result: &Result<Vec<u8>>, outfile: &Path) -> Result<Verdict> {
        match result {
            Ok(result) => {
                let mut file = self.open_file_for_read(outfile)?;
                let content = self.read_from_file(&mut file).map(|x| x.into_bytes())?;
                if &content == result {
                    Ok(Verdict::AC)
                } else {
                    Ok(Verdict::WA)
                }
            }
            Err(_e) => {
                // TODO: We need to inspect what kind of error happened.
                Ok(Verdict::RE)
            }
        }
    }
}

impl<T: ProjectExt> Project for T {
    fn check(&mut self, proj_dir: &str) -> Result<()> {
        let proj = Path::new(proj_dir);

        // Read the config file
        let config = self.read_config(proj)?;
        eprintln!("config = {:?}", config);

        check_reference_solution(&config)?;
        Ok(())
    }
    fn gen(&mut self, proj: &str) -> Result<()> {
        let proj = Path::new(proj);

        // Read the config file
        let config = self.read_config(proj)?;
        let lang_configs = config.languages;
        let TestcaseConfig { indir, .. } = config.testcase_config;
        let indir = proj.join(indir);

        // Delete all files in indir
        self.remove_dir_all(&indir)?;
        self.mkdir_p(&indir)?;

        for gen in config.generators {
            let src = proj.join(&gen.path);
            let cd = src.join("..").clean();
            let cd = self.to_absolute(&cd)?;
            let lang_config = lang_configs
                .iter()
                .find(|&c| c.language_name == gen.language_name);
            if let Some(x) = lang_config {
                let outpath = self.compile(&cd, &self.to_absolute(&src)?, &x.compile)?;
                self.run_once(&cd, &outpath, &x.run)?;
            } else {
                eprintln!("warning");
                let e = Error::ConfInvalid {
                    description: format!("language not found: {}", gen.language_name),
                };
                return Err(e);
            }
        }
        Ok(())
    }
    fn refgen(&mut self, proj_dir: &str) -> Result<()> {
        let proj_dir = Path::new(proj_dir);

        // Read the config file
        let config = self.read_config(proj_dir)?;
        let lang_configs = config.languages;
        let TestcaseConfig { indir, outdir } = config.testcase_config;
        let indir = proj_dir.join(indir);
        let outdir = proj_dir.join(outdir);

        // Do we have exactly one reference solution?
        let reference_solution_count = config
            .solutions
            .iter()
            .filter(|&solution| solution.is_reference_solution)
            .count();
        if reference_solution_count != 1 {
            let e = Error::ConfInvalid {
                description: format!(
                    "#reference solutions is not one: {}",
                    reference_solution_count
                ),
            };
            return Err(e);
        }

        // Delete all files in outdir
        self.remove_dir_all(&outdir)?;

        for sol in config.solutions {
            if !sol.is_reference_solution {
                continue;
            }
            let src = proj_dir.join(&sol.path);
            let cd = src.join("..").clean();
            let cd = self.to_absolute(&cd)?;
            let lang_config = lang_configs
                .iter()
                .find(|&c| c.language_name == sol.language_name);
            if let Some(x) = lang_config {
                let outpath = self.compile(&cd, &self.to_absolute(&src)?, &x.compile)?;
                // For all files in `indir`, generate the counterpart in `outdir`.
                for infile in self.list_dir(&indir)? {
                    eprintln!("Generating {}", infile.to_str().unwrap());
                    let outfile = outdir.join(&infile);
                    let infile = indir.join(&infile);
                    self.run_pipe(&cd, &outpath, &x.run, &infile, &outfile)?;
                }
            } else {
                eprintln!("warning");
                let e = Error::ConfInvalid {
                    description: format!("language not found: {}", sol.language_name),
                };
                return Err(e);
            }
        }
        Ok(())
    }
    fn test(&mut self, proj_dir: &str) -> Result<()> {
        let proj_dir = Path::new(proj_dir);

        // Read the config file
        let config = self.read_config(proj_dir)?;
        let lang_configs = config.languages;
        let TestcaseConfig { indir, outdir } = config.testcase_config;
        let indir = proj_dir.join(indir);
        let outdir = proj_dir.join(outdir);

        for sol in config.solutions {
            let src = proj_dir.join(&sol.path);
            let cd = src.join("..").clean();
            let cd = self.to_absolute(&cd)?;
            let lang_config = lang_configs
                .iter()
                .find(|&c| c.language_name == sol.language_name);
            if let Some(x) = lang_config {
                let outpath = self.compile(&cd, &self.to_absolute(&src)?, &x.compile)?;
                // For all files in `indir`, generate the counterpart in `outdir`.
                let mut overall_verdict = Verdict::AC;
                for infile in self.list_dir(&indir)? {
                    eprintln!("Running {}", infile.to_str().unwrap());
                    let outfile = outdir.join(&infile);
                    let infile = indir.join(&infile);

                    let result = self.run_with_input(&cd, &outpath, &x.run, &infile);
                    let verdict = self.get_verdict(&result, &outfile)?;
                    overall_verdict = std::cmp::max(overall_verdict, verdict);
                }
                if sol.expected_verdict != overall_verdict {
                    return Err(Error::VerdictMismatch {
                        expected: sol.expected_verdict,
                        actual: overall_verdict,
                    });
                } else {
                    eprintln!(
                        "Testing {} complete (result = expected = {:?})",
                        src.display(),
                        sol.expected_verdict
                    )
                }
            } else {
                eprintln!("warning");
                let e = Error::ConfInvalid {
                    description: format!("language not found: {}", sol.language_name),
                };
                return Err(e);
            }
        }

        Ok(())
    }
}

pub struct ProjectImpl;

impl IoUtilExt for ProjectImpl {}
impl RunUtilExt for ProjectImpl {}
impl ProjectExt for ProjectImpl {}

// Check if there is at most one reference solution.
fn check_reference_solution(config: &CreoConfig) -> Result<()> {
    let reference_solution_count = config
        .solutions
        .iter()
        .filter(|&solution| solution.is_reference_solution)
        .count();
    if reference_solution_count > 1 {
        let e = Error::ConfInvalid {
            description: format!(
                "#reference solutions is not <= 1: {}",
                reference_solution_count
            ),
        };
        return Err(e);
    }

    for sol in &config.solutions {
        if !sol.is_reference_solution {
            continue;
        }
        if sol.expected_verdict != Verdict::AC {
            return Err(Error::ConfInvalid {
                description: format!(
                    "the reference solution does not satisfy expected_verdict = AC: got {:?}",
                    sol.expected_verdict,
                ),
            });
        }
    }
    Ok(())
}

mod tests {
    use std::path::PathBuf;

    use super::*;

    // TODO: better testing, especially better mocking (such as obtaining multiple files' content by read_from_file)
    struct MockProject {
        processed: Vec<(String, String)>,
    }
    impl IoUtil for MockProject {
        fn open_file_for_read(&self, _filepath: &Path) -> Result<Box<dyn std::io::Read>> {
            Ok(Box::new(b"don't care" as &[u8]))
        }
        fn read_from_file(&self, _file: &mut dyn std::io::Read) -> Result<String> {
            Ok(r#"
[[generators]]
language_name = "C++"
path = "gen.cpp"

[[languages]]
language_name = "C++"
target_ext = ".cpp"
compile = ["g++", "-O2", "-std=gnu++11", "-o", "$OUT", "$IN"]
run = ["./a.out"]

[[solutions]]
path = "sol.cpp"
language_name = "C++"
is_reference_solution = true
"#
            .to_string())
        }
        fn mkdir_p(&mut self, _path: &Path) -> Result<()> {
            Ok(())
        }
        fn to_absolute(&self, _path: &Path) -> Result<PathBuf> {
            Ok("gen-absolute".into())
        }
        fn list_dir(&self, _path: &Path) -> Result<Vec<PathBuf>> {
            Ok(vec!["a".into(), "b".into()])
        }
        fn remove_dir_all(&self, _path: &Path) -> Result<()> {
            Ok(())
        }
    }
    impl RunUtil for MockProject {
        fn compile(&mut self, _cd: &Path, src: &Path, _compile: &[String]) -> Result<PathBuf> {
            assert_eq!(src, PathBuf::from("gen-absolute"));
            Ok("outpath".into())
        }
        fn run_once(&mut self, _cd: &Path, exec: &Path, _run: &[String]) -> Result<()> {
            assert_eq!(exec, PathBuf::from("outpath"));
            Ok(())
        }
        fn run_with_input(
            &mut self,
            _cd: &Path,
            _exec: &Path,
            _run: &[String],
            _infile: &Path,
        ) -> Result<Vec<u8>> {
            Ok((b"aa" as &[u8]).to_owned())
        }
        fn run_pipe(
            &mut self,
            _cd: &Path,
            _exec: &Path,
            _run: &[String],
            infile: &Path,
            outfile: &Path,
        ) -> Result<()> {
            self.processed.push((
                infile.to_str().unwrap().to_owned(),
                outfile.to_str().unwrap().to_owned(),
            ));
            Ok(())
        }
    }
    impl ProjectExt for MockProject {}

    #[test]
    fn check_reference_solution_works() {
        use crate::entity::sol::{SolutionConfig, Verdict};
        let mut config = CreoConfig::default();
        config.solutions = vec![SolutionConfig {
            path: "".to_owned(),
            language_name: "".to_owned(),
            expected_verdict: Verdict::AC,
            is_reference_solution: true,
        }];
        check_reference_solution(&config).unwrap();
    }

    #[test]
    fn check_reference_solution_fails_if_two_reference_solutions_exist() {
        use crate::entity::sol::{SolutionConfig, Verdict};
        let mut config = CreoConfig::default();
        config.solutions = vec![
            SolutionConfig {
                path: "".to_owned(),
                language_name: "".to_owned(),
                expected_verdict: Verdict::AC,
                is_reference_solution: true,
            };
            2
        ];

        let e = check_reference_solution(&config).unwrap_err();
        let desc = e.to_string();
        // We want an error message that indicates there are 2 reference solutions.
        assert!(desc.contains("reference solution"), "desc = {}", desc);
        assert!(desc.contains('2'), "desc = {}", desc);
    }

    #[test]
    fn check_reference_solution_fails_if_reference_solutions_expected_verdict_is_not_ac() {
        use crate::entity::sol::{SolutionConfig, Verdict};
        let mut config = CreoConfig::default();
        config.solutions = vec![SolutionConfig {
            path: "".to_owned(),
            language_name: "".to_owned(),
            expected_verdict: Verdict::WA,
            is_reference_solution: true,
        }];

        let e = check_reference_solution(&config).unwrap_err();
        let desc = e.to_string();
        // We want an error message that indicates the reference solution should have expected_verdict AC,
        // but in fact it was WA.
        assert!(desc.contains("reference solution"), "desc = {}", desc);
        assert!(desc.contains("WA"), "desc = {}", desc);
        assert!(desc.contains("AC"), "desc = {}", desc);
    }

    #[test]
    fn gen_project_works() {
        let mut project = MockProject { processed: vec![] };
        project.gen(".").unwrap();
    }

    #[test]
    fn refgen_project_works() {
        let mut project = MockProject { processed: vec![] };
        project.refgen(".").unwrap();
        assert_eq!(
            project.processed,
            vec![
                ("./in/a".to_owned(), "./out/a".to_owned()),
                ("./in/b".to_owned(), "./out/b".to_owned()),
            ]
        );
    }

    #[test]
    fn test_project_works() {
        let mut project = MockProject { processed: vec![] };
        // TODO: explain why RE != AC is returned
        let result = project.test(".");
        // We use a pattern matching because Error can't implement PartialEq
        // (because of std::io::Error, which doesn't implement PartialEq)
        if let Err(Error::VerdictMismatch { expected, actual }) = result {
            assert_eq!(expected, Verdict::AC);
            assert_eq!(actual, Verdict::WA);
        } else {
            unreachable!("unreachable: the assertion above does not hold");
        }
    }
}
