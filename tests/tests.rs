#![cfg_attr(feature = "fatal-warnings", deny(warnings))]
#![cfg(not(miri))]

extern crate compiletest_rs as compiletest;

use std::path::PathBuf;

fn find_rlib(dependency_path: &str, dependency_name: &str) -> std::io::Result<Option<PathBuf>> {
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

fn rustc_flags(dependency_path: &str, dependencies: &[&str]) -> String {
    let mut flags = format!("--edition=2021 -L dependency={} ", dependency_path);

    for dep in dependencies {
        let rlib_path =
            find_rlib(dependency_path, dbg!(dep)).expect("io error").expect("rlib not found");

        flags.push_str(&format!("--extern {}={} ", dep, rlib_path.display()));
    }

    flags
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

    config.target_rustcflags = Some(rustc_flags("target/debug/deps/", &dependencies));

    compiletest::run_tests(&config);
}
