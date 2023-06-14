use assert_cmd::Command;

type TestResult = Result<(), Box<dyn std::error::Error>>;

const CMD: &str = "rjp";


fn run_stdin(args: &[&str], stdin: &str, expected: &str) -> TestResult {
    Command::cargo_bin(CMD)
        .unwrap()
        .args(args)
        .write_stdin(stdin)
        .assert()
        .success()
        .stdout(expected.to_owned());
    Ok(())
}

#[test]
fn prints_stdin() -> TestResult {
    run_stdin(&[], "{}", "{}\n")
}
