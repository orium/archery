#![cfg_attr(feature = "fatal-warnings", deny(warnings))]

extern crate compiletest_rs as compiletest;

#[cfg(not(miri))]
#[test]
fn compile_tests() {
    use compiletest::common::ConfigWithTemp;
    use compiletest::common::Mode;
    use compiletest::Config;
    use std::path::PathBuf;

    let mut config: ConfigWithTemp = Config::default().tempdir();

    config.mode = Mode::CompileFail;
    config.src_base = PathBuf::from("tests/compile-fail");
    config.target_rustcflags = Some("-L target/debug -L target/debug/deps".to_string());
    config.clean_rmeta();

    compiletest::run_tests(&config);
}
