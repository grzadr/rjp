// src/args.rs

use super::element::{Element, Selects};
use clap::{Parser, ValueEnum};
use std::error::Error;
use std::str::FromStr;
use stderrlog::Timestamp;

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum Join {
    Merge,
    Concat,
}

#[derive(Debug)]
pub struct Config {
    pub files: Vec<String>,
    pub selects: Selects,
    pub filters: Vec<String>,
    pub verbosity: usize,
    pub ts: Timestamp,
    pub indent: usize,
    pub join: Join,
    pub expand: bool
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[arg(help = "Input file", default_values_t = vec!["-".to_string()])]
    files: Vec<String>,

    #[arg(
        short, long, help = "Elements to be selected",
        value_parser = clap::builder::ValueParser::new(Element::from_str),
        // default_value_t = Some(Selects::new(vec![Select::from_str(".").unwrap()]))
    )]
    selects: Vec<Element>,

    #[arg(short, long, help = "Filtering conditions")]
    filters: Vec<String>,

    #[arg(short, long, help = "Size of indentation",
        value_parser = clap::value_parser!(i32).range(1..),
        default_value = "4"
    )]
    indent: Option<i32>,

    #[arg(short, long, help = "Shows additional debug info")]
    debug: bool,

    #[arg(short, long, action = clap::ArgAction::Count)]
    verbosity: u8,

    #[arg(short, long, value_enum, help = "Strategy for merging values")]
    join: Join,

    #[arg(short, long, help = "Expand values instead of producing JSON")]
    expand: bool,
}

pub fn get_args() -> MyResult<Config> {
    let cli = Cli::parse();

    if cli.debug {
        dbg!(&cli);
    }

    let verbosity = (if cli.debug {
        4
    } else {
        std::cmp::min(cli.verbosity, 4)
    }) as usize;
    let ts = stderrlog::Timestamp::from_str("ms").unwrap();
    let selects = Selects::new(if cli.selects.is_empty() {
        vec![Element::from_str("*").unwrap()]
    } else {
        cli.selects
    });
    let indent = usize::try_from(cli.indent.unwrap()).unwrap();

    Ok(Config {
        files: cli.files,
        selects,
        filters: cli.filters,
        verbosity,
        ts,
        indent,
        join: cli.join,
        expand: cli.expand
    })
}
