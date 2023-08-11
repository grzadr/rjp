mod args;
mod select;
use serde_json::Value;
use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use select::{SelectedValue, Selects};
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

fn select_values(json_content: &Value, selects: &Selects) -> MyResult<Vec<SelectedValue>> {
    let mut selected: Vec<SelectedValue> = Vec::new();

    for select in &selects.0{
        selected.push(SelectedValue::new(json_content.clone(), select.clone() ));
    }

    Ok(selected)
}

fn merge_values(selected: Vec<SelectedValue>) -> MyResult<Value> {
    Ok(Value::Null)
}

fn print_selected_values(selected: Vec<SelectedValue>, format: &str, indent: usize) -> MyResult<()> {
    match format {
        "json" => {
            let mut json_content = json!({});
            for mut selected_value in selected {
                debug!("{}", &selected_value.value);
                dbg!(&selected_value);
                let content = json_content.as_object_mut().unwrap();
                if selected_value.path.name() == "." {
                    content.append(&mut selected_value.value.as_object_mut().unwrap());
                } else {
                    content.insert(selected_value.path.name().to_string(), selected_value.value);
                }
            }
            println!("{}", format_json(json_content, indent)?);
        }
        "text" => {
            for selected_value in selected {
                println!("{}", selected_value.value);
            }
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

    let mut selected: Vec<SelectedValue> = Vec::new();

    for filename in config.files {
        info!("Processing file {filename}");
        let json_content = load_json(&filename)?;
        debug!("{}", serde_json::to_string_pretty(&json_content)?);
        selected.append(&mut select_values(&json_content, &config.selects)?);
    }

    let _ = print_selected_values(selected, "json", config.indent);

    Ok(())
}

pub fn run_from_args() -> MyResult<()> {
    args::get_args().and_then(run)
}
