use assert_cmd::Command;
use std::fs;

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

fn run_file_success(input_path: &str, args: &[&str]) -> TestResult {
    let input_file = format!("{}/{}", input_path, "input.json");
    let expected_file = format!("{}/{}", input_path, "expected.json");
    let expected = fs::read_to_string(&expected_file).expect("Failed to read expected file");
    let input = fs::read_to_string(&input_file).expect("Failed to read input file");

    assert!(expected.len() > 0);
    assert!(input.len() > 0);

    Command::cargo_bin(CMD)
        .unwrap()
        .args(args)
        .args(&["-d", "-i", &input_file])
        .assert()
        .success()
        .stdout(expected.to_owned());
    Ok(())
}

#[test]
fn test_empty_file() -> TestResult {
    run_file_success("tests/json_files/empty", &[])
}
