use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about)]
/// A Rust version of the 'cat' command
struct Args {
    /// The file to read from
    #[arg(required(true))]
    file: String,

    #[arg(short('b'))]
    non_blank_lines: bool,
}
fn main() {
    return;
}
