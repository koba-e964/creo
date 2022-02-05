use assert_cmd::cmd::Command;
use assert_fs::prelude::*;
use predicates::prelude::*;

#[test]
fn init_works() {
    // Asserts that `creo init .` works.
    let temp = assert_fs::TempDir::new().unwrap();
    Command::cargo_bin("creo")
        .unwrap()
        .current_dir(&temp)
        .args(&["init", "."])
        .unwrap();
    temp.child("creo.toml").assert(predicate::path::is_file());

    temp.close().unwrap();
}
