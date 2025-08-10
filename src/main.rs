use clap::Parser;

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
fn main() {
    let args = Args::parse();
    println!("{args:#?}");
}
