// src/args.rs

use super::select::{Selects, Select};
use clap::Parser;
use stderrlog::Timestamp;
use std::str::FromStr;
use std::error::Error;

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug)]
pub struct Config {
    pub files: Vec<String>,
    pub selects: Selects,
    pub filters: Vec<String>,
    pub verbosity: usize,
    pub ts: Timestamp,
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[arg(help = "Input file", default_values_t = vec!["-".to_string()])]
    files: Vec<String>,

    #[arg(
        short, long, help = "Elements to be selected",
        value_parser = clap::builder::ValueParser::new(Select::from_str),
        // default_value_t = Some(Selects::new(vec![Select::from_str(".").unwrap()]))
    )]
    selects: Vec<Select>,

    #[arg(short, long, help = "Filtering conditions")]
    filters: Vec<String>,

    #[arg(short, long, help = "Shows additional debug info")]
    debug: bool,

    #[arg(short, long, action = clap::ArgAction::Count)]
    verbosity: u8,
}

pub fn get_args() -> MyResult<Config> {
    let cli = Cli::parse();

    if cli.debug {
        dbg!(&cli);
    }

    let verbosity = (if cli.debug {4} else {std::cmp::min(cli.verbosity, 4)}) as usize;
    let ts = stderrlog::Timestamp::from_str("ms").unwrap();
    let selects = Selects::new(if cli.selects.is_empty() {vec![Select::from_str(".").unwrap()]} else {cli.selects});

    Ok(Config {
        files: cli.files,
        selects,
        filters: cli.filters,
        verbosity,
        ts
    })
}