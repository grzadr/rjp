mod input_source;

use clap::Parser;
use input_source::InputSource;
use serde_json::Value;

use log::*;
use std::str::FromStr;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[arg(short, long, help = "Input file")]
    input: Option<String>,

    #[arg(short, long, help = "Elements to be selected")]
    select: Option<String>,

    #[arg(short, long, help = "Filtering conditions")]
    filter: Option<String>,

    #[arg(short, long, help = "Shows additional debug info")]
    debug: bool,

    #[arg(short, long, action = clap::ArgAction::Count)]
    verbosity: u8,
}
fn main() {
    info!("Start rjp");
    let cli = Cli::parse();
    let verbose = std::cmp::min(cli.verbosity, 4) as usize;
    let ts = stderrlog::Timestamp::from_str("ms").unwrap();

    stderrlog::new()
        .module(module_path!())
        .quiet(false)
        .verbosity(verbose)
        .timestamp(ts)
        .init()
        .unwrap();

    debug!("{:#?}", cli);

    let mut input_source = if let Some(path) = cli.input {
        InputSource::new_file(&path).expect(&format!("Error reading {}", &path))
    } else {
        InputSource::new_stdin()
    };

    let mut all_lines = String::new();
    loop {
        let line = input_source.lines().expect("Error reading line");
        if line.is_empty() {
            break;
        }
        all_lines.push_str(&line);
    }

    debug_assert!(all_lines.len() > 0);

    let json_content: Value = serde_json::from_str(&all_lines).expect("");

    info!("Printing output");
    println!("{}", serde_json::to_string_pretty(&json_content).unwrap())

}
