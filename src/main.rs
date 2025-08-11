use anyhow::Result;
use clap::Parser;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

#[derive(Parser, Debug)]
#[command(author, version, about)]
/// A Rust version of the 'cat' command
struct Args {
    /// The file to read from
    #[arg(value_name = "FILE", default_value = "-")]
    files: Vec<String>,

    /// Print line numbers
    #[arg(short('n'), long("number"), conflicts_with("non_blank_lines"))]
    line_numbers: bool,

    /// Print the line numbers of non-blank lines
    #[arg(short('b'), long("number-nonblank"))]
    non_blank_lines: bool,
}
fn run(args: Args) -> Result<()> {
    for filename in args.files {
        let file = match open(&filename) {
            Ok(file) => file,
            Err(e) => {
                eprintln!("{e}");
                continue;
            }
        };
        let mut line_number = 1;

        for line in file.lines() {
            let line = line?;
            if args.line_numbers {
                println!("     {line_number} {line}");
                line_number += 1;
            } else if args.non_blank_lines {
                if line.trim().is_empty() {
                    println!();
                } else {
                    println!("     {line_number} {line}");
                    line_number += 1;
                }
            } else {
                println!("{line}");
            }
        }
    }
    Ok(())
}
fn open(filename: &str) -> Result<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}
fn main() {
    if let Err(e) = run(Args::parse()) {
        eprintln!("{e}");
        std::process::exit(1);
    }
}
