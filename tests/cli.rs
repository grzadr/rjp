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
        .args(&["-d", &input_file])
        .assert()
        .success()
        .stdout(expected.to_owned());
    Ok(())
}

#[test]
fn test_empty_file() -> TestResult {
    run_file_success("tests/json_files/empty", &[])
}

#[test]
fn test_empty_array() -> TestResult {
    run_file_success("tests/json_files/empty_array", &[])
}

#[test]
fn test_select_default() -> TestResult {
    run_file_success("tests/json_files/select_default", &[])
}

#[test]
fn test_select_name() -> TestResult {
    run_file_success("tests/json_files/select_name", &["-s", "name"])
}

#[test]
fn test_select_name_price_sku_() -> TestResult {
    run_file_success("tests/json_files/select_name", &["-s", "name","-s", "price","-s", "sku",])
}

#[test]
fn test_select_ship_to() -> TestResult {
    run_file_success("tests/json_files/select_name", &["-s", "shipTo"])
}
