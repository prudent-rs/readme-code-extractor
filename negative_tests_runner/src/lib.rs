use snapbox::cmd::Command;

#[test]
fn simple_no_panic() {
    let cmd = Command::new("cargo").arg("build").arg("--release");
    let cmd = cmd.current_dir( std::fs::canonicalize() );
    //t.compile_fail("../negative_tests/src/bin/simple_no_panic.rs");
}