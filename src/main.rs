mod input_source;

use input_source::InputSource;
use clap::Parser;
use serde_json:: Value;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[arg(help = "Input file")]
    file: Option<String>,

    #[arg(short, long, help = "Elements to be selected")]
    select: Option<String>,

    #[arg(short, long, help = "Filtering conditions")]
    filter: Option<String>,

    #[arg(short, long, help = "Shows additional debug info")]
    debug: bool
}
fn main() {
    let cli = Cli::parse();

    if cli.debug {
        eprintln!("{:#?}", cli)
    }

    let mut input_source = if let Some(path) = cli.file {
        InputSource::new_file(&path).expect(&format!("Error reading {}", &path))
    } else {
        InputSource::new_stdin()
    };

    let mut all_lines = String::new();
    loop {
        let line = input_source
            .lines()
            .expect("Error reading line");
        if line.is_empty() {
            break;
        }
        all_lines.push_str(&line);
    }

    let json_content: Value = serde_json::from_str(&all_lines).expect("");

    println!("{}", serde_json::to_string_pretty(&json_content).unwrap())
}
