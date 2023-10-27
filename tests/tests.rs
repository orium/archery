#![cfg_attr(feature = "fatal-warnings", deny(warnings))]
#![cfg(not(miri))]

extern crate compiletest_rs as compiletest;

use std::path::{Path, PathBuf};

fn find_rlib(dependency_path: &Path, dependency_name: &str) -> std::io::Result<Option<PathBuf>> {
    use std::fs::read_dir;

    for entry in read_dir(dependency_path)? {
        let entry = entry?;

        if let Some(filename) = entry.path().file_name().and_then(|f| f.to_str()) {
            if filename.starts_with(&format!("lib{}", dependency_name))
                && filename.ends_with(".rlib")
            {
                return Ok(Some(entry.path()));
            }
        }
    }

    Ok(None)
}

fn rustc_flags(dependency_path: &Path, dependencies: &[&str]) -> String {
    let mut flags = format!("--edition=2021 -L dependency={} ", dependency_path.display());

    for dep in dependencies {
        let rlib_path = find_rlib(dependency_path, dep).expect("io error").expect("rlib not found");

        flags.push_str(&format!("--extern {}={} ", dep, rlib_path.display()));
    }

    flags
}

fn dependency_path() -> PathBuf {
    std::env::args()
        .next()
        .map(PathBuf::from)
        .and_then(|p| p.parent().map(ToOwned::to_owned))
        .expect("could not find dependency path")
}

#[test]
fn compile_tests() {
    use compiletest::common::Mode;
    use compiletest::Config;
    use std::path::PathBuf;

    let mut config: Config = Config::default();

    config.mode = Mode::CompileFail;
    config.src_base = PathBuf::from("tests/compile-fail");

    let dependencies = ["archery", "static_assertions"];

    config.target_rustcflags = Some(rustc_flags(&dependency_path(), &dependencies));

    compiletest::run_tests(&config);
}
