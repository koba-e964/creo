use path_clean::PathClean;
use std::ffi::OsString;
use std::path::Path;

use crate::entity::config::CreoConfig;
use crate::entity::sol::Verdict;
use crate::entity::testcase::TestcaseConfig;
use crate::entity::val::ValidatorConfig;
use crate::error::{Error, Result};
use crate::io_util::{IoUtil, IoUtilExt};
use crate::run_util::{RunUtil, RunUtilExt};

/// A trait that provides functions to handle a project directory.
pub trait Project {
    /// Add a new entity.
    #[allow(unused)]
    fn add(&mut self, proj_dir: &str, ty: &str, name: &str) -> Result<()> {
        unreachable!();
    }
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
    /// Validate all input files.
    #[allow(unused)]
    fn val(&mut self, proj_dir: &str) -> Result<()> {
        unreachable!();
    }
}

pub trait ProjectExt: IoUtil + RunUtil {
    fn read_config(&mut self, proj: &Path) -> Result<CreoConfig> {
        // Read the config file
        let config_filepath = proj.join("creo.toml");
        let mut file = self.open_file_for_read(&config_filepath)?;
        let content = self.read_from_file(&mut file)?;
        let config: CreoConfig = toml::from_str(&content)?;

        Ok(config)
    }

    fn write_config(&mut self, proj: &Path, config: &CreoConfig) -> Result<()> {
        // Read the config file
        let config_filepath = proj.join("creo.toml");
        let mut file = self.open_file_for_write(&config_filepath)?;
        self.write_str_to_file(&mut file, &toml::to_string(config)?)?;

        Ok(())
    }

    // Given the result of execution and the expected output file, find the verdict.
    fn get_verdict(&mut self, result: &Result<Vec<u8>>, outfile: &Path) -> Result<Verdict> {
        match result {
            Ok(result) => {
                let mut file = self.open_file_for_read(outfile)?;
                let content = self.read_bytes_from_file(&mut file)?;
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
    fn add(&mut self, proj_dir: &str, ty: &str, name: &str) -> Result<()> {
        let proj = Path::new(proj_dir);

        // Read the config file
        let mut config = self.read_config(proj)?;
        let lang_configs = config.languages.clone();
        let name = self.to_absolute(Path::new(&name))?;
        let ext = name.extension();
        let mut lang = None;
        let mut lang_conf = None;
        for l in lang_configs {
            if ext == Some(&OsString::from(l.target_ext.clone())) {
                lang = Some(l.language_name.clone());
                lang_conf = Some(l.clone());
            }
        }
        let (lang, lang_conf) = if let (Some(lang), Some(lang_conf)) = (lang, lang_conf) {
            (lang, lang_conf)
        } else {
            return Err(Error::ConfInvalid {
                description: "extension not registered in the conf file".to_owned(),
            });
        };
        // same directory, same name with the extension replaced with .sh
        let mut script_filepath = name.clone();
        script_filepath.set_extension("sh");
        if ty == "val" {
            let mut shfile = self.create_file_if_nonexistent(&script_filepath, 0o755)?;
            self.create_file_if_nonexistent(&name, 0o644)?;
            let file_stem = name.file_stem().unwrap().to_str().unwrap();
            let in_dir = config.testcase_config.indir.clone();
            let cmd = self.build_command(&lang_conf.compile, &name, Path::new(file_stem));
            let mut cmd = cmd.iter().fold("".to_owned(), |x, y| x + y + " ");
            cmd.pop();
            // TODO: support other directories
            self.write_str_to_file(
                &mut shfile,
                &format!(
                    r#"#!/bin/bash
{}
for file in ../{}/*; do
    echo ${{file}}
    ./{} <${{file}}
done
"#,
                    cmd, in_dir, file_stem
                ),
            )?;
            config.validators.push(ValidatorConfig {
                path: name.into_os_string().into_string().unwrap(),
                language_name: lang,
            });
            self.write_config(proj, &config)?;
            return Ok(());
        }
        if ty == "gen" {
            panic!()
        }
        if ty == "sol" {
            panic!()
        }
        Err(Error::UnknownEntityType {
            entity_type: ty.to_owned(),
        })
    }
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
                    eprint!("Running {}", infile.to_str().unwrap());
                    let outfile = outdir.join(&infile);
                    let infile = indir.join(&infile);

                    let result = self.run_with_input(&cd, &outpath, &x.run, &infile);
                    let verdict = self.get_verdict(&result, &outfile)?;
                    overall_verdict = std::cmp::max(overall_verdict, verdict.clone());
                    eprintln!(" {:?} (overall: {:?})", verdict, overall_verdict);
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

    fn val(&mut self, proj_dir: &str) -> Result<()> {
        let proj_dir = Path::new(proj_dir);

        // Read the config file
        let config = self.read_config(proj_dir)?;
        let lang_configs = config.languages;
        let TestcaseConfig { indir, .. } = config.testcase_config;
        let indir = proj_dir.join(indir);

        if config.validators.is_empty() {
            return Err(Error::ConfInvalid {
                description: "Validators not found in creo.toml".to_owned(),
            });
        }

        for val in config.validators {
            let src = proj_dir.join(&val.path);
            let cd = src.join("..").clean();
            let cd = self.to_absolute(&cd)?;
            let lang_config = lang_configs
                .iter()
                .find(|&c| c.language_name == val.language_name);
            if let Some(x) = lang_config {
                let outpath = self.compile(&cd, &self.to_absolute(&src)?, &x.compile)?;
                // Validate each file in `indir`.
                for infile in self.list_dir(&indir)? {
                    eprintln!("Validating {}", infile.to_str().unwrap());
                    let infile = indir.join(&infile);
                    if let Err(e) = self.run_with_input(&cd, &outpath, &x.run, &infile) {
                        // A hack to check if the subprocess exited with status code != 0.
                        if let Error::IOError(ref inner) = e {
                            if inner.kind() == std::io::ErrorKind::InvalidData {
                                let inner = match e {
                                    Error::IOError(inner) => inner,
                                    _ => unreachable!(),
                                };

                                return Err(Error::ValidationFailed {
                                    validator: val.path.clone(),
                                    infile: infile.display().to_string(),
                                    inner: Box::new(inner) as Box<dyn std::error::Error>,
                                });
                            }
                        }
                        return Err(e);
                    }
                }
            } else {
                eprintln!("warning");
                let e = Error::ConfInvalid {
                    description: format!("language not found: {}", val.language_name),
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
        fn create_file_if_nonexistent(
            &mut self,
            _filepath: &Path,
            _mode: u32,
        ) -> Result<Box<dyn std::io::Write>> {
            Ok(Box::new(vec![]))
        }
        fn open_file_for_read(&self, _filepath: &Path) -> Result<Box<dyn std::io::Read>> {
            Ok(Box::new(b"don't care" as &[u8]))
        }
        fn open_file_for_write(&self, _filepath: &Path) -> Result<Box<dyn std::io::Write>> {
            Ok(Box::new(vec![]))
        }
        fn read_from_file(&self, _file: &mut dyn std::io::Read) -> Result<String> {
            Ok(r#"
[[generators]]
language_name = "C++"
path = "gen.cpp"

[[languages]]
language_name = "C++"
target_ext = "cpp"
compile = ["g++", "-O2", "-std=gnu++11", "-o", "$OUT", "$IN"]
run = ["./a.out"]

[[solutions]]
path = "sol.cpp"
language_name = "C++"
is_reference_solution = true

[[validators]]
path = "val.cpp"
language_name = "C++"
"#
            .to_string())
        }
        fn read_bytes_from_file(&self, _file: &mut dyn std::io::Read) -> Result<Vec<u8>> {
            Ok(b"correct output".to_vec())
        }
        fn write_str_to_file(&self, _file: &mut dyn std::io::Write, _s: &str) -> Result<()> {
            Ok(())
        }
        fn mkdir_p(&mut self, _path: &Path) -> Result<()> {
            Ok(())
        }
        fn to_absolute(&self, _path: &Path) -> Result<PathBuf> {
            Ok("gen-absolute.cpp".into())
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
            assert_eq!(src, PathBuf::from("gen-absolute.cpp"));
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
            Ok((b"wrong output" as &[u8]).to_owned())
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
        fn build_command(&self, _run: &[String], _infile: &Path, _outfile: &Path) -> Vec<String> {
            vec!["gcc".to_owned()]
        }
    }
    impl ProjectExt for MockProject {}

    #[test]
    fn check_reference_solution_works() {
        use crate::entity::sol::{SolutionConfig, Verdict};
        let config = CreoConfig {
            solutions: vec![SolutionConfig {
                path: "".to_owned(),
                language_name: "".to_owned(),
                expected_verdict: Verdict::AC,
                is_reference_solution: true,
            }],
            ..Default::default()
        };
        check_reference_solution(&config).unwrap();
    }

    #[test]
    fn check_reference_solution_fails_if_two_reference_solutions_exist() {
        use crate::entity::sol::{SolutionConfig, Verdict};
        let config = CreoConfig {
            solutions: vec![
                SolutionConfig {
                    path: "".to_owned(),
                    language_name: "".to_owned(),
                    expected_verdict: Verdict::AC,
                    is_reference_solution: true,
                };
                2
            ],
            ..Default::default()
        };

        let e = check_reference_solution(&config).unwrap_err();
        let desc = e.to_string();
        // We want an error message that indicates there are 2 reference solutions.
        assert!(desc.contains("reference solution"), "desc = {}", desc);
        assert!(desc.contains('2'), "desc = {}", desc);
    }

    #[test]
    fn check_reference_solution_fails_if_reference_solutions_expected_verdict_is_not_ac() {
        use crate::entity::sol::{SolutionConfig, Verdict};
        let config = CreoConfig {
            solutions: vec![SolutionConfig {
                path: "".to_owned(),
                language_name: "".to_owned(),
                expected_verdict: Verdict::WA,
                is_reference_solution: true,
            }],
            ..Default::default()
        };

        let e = check_reference_solution(&config).unwrap_err();
        let desc = e.to_string();
        // We want an error message that indicates the reference solution should have expected_verdict AC,
        // but in fact it was WA.
        assert!(desc.contains("reference solution"), "desc = {}", desc);
        assert!(desc.contains("WA"), "desc = {}", desc);
        assert!(desc.contains("AC"), "desc = {}", desc);
    }

    #[test]
    fn add_project_works() {
        let mut project = MockProject { processed: vec![] };
        project.add(".", "val", "test.cpp").unwrap();
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

    #[test]
    fn val_project_works() {
        let mut project = MockProject { processed: vec![] };
        let result = project.val(".");
        result.unwrap();
    }
}
