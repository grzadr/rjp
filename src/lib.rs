mod args;
mod element;
use serde_json::Value;
use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use element::{SelectedValue, Selects};
use serde::Serialize;
use serde_json::json;


use log::*;

type MyResult<T> = Result<T, Box<dyn Error>>;

fn open(filename: &str) -> MyResult<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}

fn load_json(filename: &str) -> MyResult<Value> {
    let file = open(filename).map_err(|e| format!("Error reading {}: {}", filename, e))?;
    let json_content: Value = serde_json::from_reader(file)?;
    Ok(json_content)
}

fn format_json(json_content: Value, indent: usize) -> MyResult<String> {
    let mut buf = Vec::new();
    let indent = std::iter::repeat(" ").take(indent).collect::<String>();
    let formatter = serde_json::ser::PrettyFormatter::with_indent(indent.as_bytes());
    let mut ser = serde_json::Serializer::with_formatter(&mut buf, formatter);
    
    json_content.serialize(&mut ser).unwrap();
    Ok(String::from_utf8(buf).unwrap())
}

fn select_values(filename: String, json_content: &Value, selects: &Selects) -> MyResult<Vec<SelectedValue>> {
    let mut selected: Vec<SelectedValue> = Vec::new();

    for select in &selects.0{
        selected.push(SelectedValue::new(filename, json_content.clone(), select.clone()));
    }

    Ok(selected)
}

fn process_files(files: Vec<String>, fields: &Selects) -> MyResult<Vec<SelectedValue>> {
    let mut selected: Vec<SelectedValue> = Vec::new();

    for filename in files {
        info!("Processing file {filename}");
        let json_content = load_json(&filename)?;
        debug!("{}", serde_json::to_string_pretty(&json_content)?);
        selected.append(&mut select_values(filename, &json_content, fields)?);
    }

    Ok(selected)
}


fn join_values(selected: Vec<SelectedValue>, join: args::Join) -> Option<Value> {

    if selected.is_empty() {
        return None;
    }

    let mut result = match selected.first().unwrap().value {
        Value::Array(v) => v,
        Value::Object(v) => v,
        v => 
    }

    for ele in selected {
        let value = ele.value;
        let path = ele.path;
        match &value {
            Value::Array => 
        }
    }

    result
}

fn print_output(value: Option<Value>, format: &str, indent: usize) -> MyResult<()> {
    if value.is_none() {

    }
    match format {
        "json" => {
            println!("{}", format_json(value.unwrap(), indent)?);
        }
        "text" => {
            todo!("tesxt not implemented")
        }
        _ => {
            return Err(format!("Unknown format {}", format).into());
        }
    }
    Ok(())
}

pub fn run(config: args::Config) -> MyResult<()> {
    stderrlog::new()
        .module(module_path!())
        .quiet(false)
        .verbosity(config.verbosity)
        .timestamp(config.ts)
        .init()
        .unwrap();

    info!("Initialization");

    debug!("Configuration {:#?}", config);

    let selected = process_files(config.files, &config.selects);

    let merged = join_values(selected?, config.join);

    print_output(merged, "json", config.indent)?;

    Ok(())
}

pub fn run_from_args() -> MyResult<()> {
    args::get_args().and_then(run)
}
