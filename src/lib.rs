mod args;
mod select;
use serde_json::Value;
use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use select::{Select, Selects};

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

fn process_json(json_content: Value, selects: &Selects, filters: &Vec<String>) -> MyResult<()> {
    let mut selected = json_content;

    println!("{}", serde_json::to_string_pretty(&selected).unwrap());
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
        process_json(json_content, &config.selects, &config.filters)?;
    }

    Ok(())
}

pub fn run_from_args() -> MyResult<()> {
    let config = args::get_args()?;
    run(config)
}
