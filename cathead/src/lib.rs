use clap::Parser;
use std::error::Error;

#[derive(Debug)]
pub struct Config {
    files: Vec<String>,
    lines: usize,
    bytes: Option<usize>,
}

#[derive(Parser, Debug)]
#[command(author = "Vermee81")]
#[command(version, about = "Show some head lines of a file")]
struct Args {
    #[arg(default_value = "-", value_name = "FILE")]
    files: Vec<String>,
    #[arg(short = 'n', long, default_value = "10", value_name = "LINES")]
    lines: usize,
    #[arg(short = 'c', long, value_name = "BYTES", conflicts_with("lines"))]
    bytes: Option<usize>,
}

type MyResult<T> = Result<T, Box<dyn Error>>;

pub fn get_args() -> MyResult<Config> {
    let args = Args::parse();
    let files = args.files;

    Ok(Config {
        files,
        lines: args.lines,
        bytes: args.bytes,
    })
}

pub fn run(config: Config) -> MyResult<()> {
    println!("{:#?}", config);
    Ok(())
}
