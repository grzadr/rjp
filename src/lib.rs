mod args;
mod select;
use serde_json::Value;
use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use select::{Select, Selects};
use serde_json::json;
use serde::Serialize;
use std::string::FromUtf8Error;

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

fn format_json(mut json_content: Value, indent: usize) -> MyResult<String> {
    let mut buf = Vec::new();
    let indent = std::iter::repeat(" ").take(indent).collect::<String>();
    let formatter = serde_json::ser::PrettyFormatter::with_indent(indent.as_bytes());
    let mut ser = serde_json::Serializer::with_formatter(&mut buf, formatter);
    
    json_content.serialize(&mut ser).unwrap();
    Ok(String::from_utf8(buf).unwrap())
}

fn process_json(json_content: Value, selects: &Selects, filters: &Vec<String>, indent: usize) -> MyResult<()> {
    let mut selected = json_content;

    println!("{}", format_json(selected, indent).unwrap());
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

    for filename in config.files {
        info!("Processing file {}", filename);
        let json_content = load_json(&filename)?;
        process_json(json_content, &config.selects, &config.filters, config.indent)?;
    }

    Ok(())
}

pub fn run_from_args() -> MyResult<()> {
    let config = args::get_args()?;
    run(config)
}
