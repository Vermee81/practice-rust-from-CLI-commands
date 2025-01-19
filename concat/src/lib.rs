use clap::Parser;
use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

#[derive(Debug)]
pub struct Config {
    files: Vec<String>,
    number_lines: bool,
    number_nonblank_lines: bool,
}

#[derive(Parser, Debug)]
#[command(author = "Vermee81")]
#[command(version, about = "Concatenate files to standard output with MEOW")]
struct Args {
    #[arg(required = false, default_value = "-", help = "Input files")]
    files: Vec<String>,
    #[arg(
        short = 'n',
        long = "number",
        help = "Number lines",
        conflicts_with = "number_nonblank_lines"
    )]
    number_lines: bool,
    #[arg(short = 'b', long = "number_nonblank", help = "Number nonblank lines")]
    number_nonblank_lines: bool,
}

type MyResult<T> = Result<T, Box<dyn Error>>;

fn open(filename: &str) -> MyResult<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}

pub fn get_args() -> MyResult<Config> {
    let args = Args::parse();
    let files = args.files;

    Ok(Config {
        files,
        number_lines: args.number_lines,
        number_nonblank_lines: args.number_nonblank_lines,
    })
}

pub fn run(config: Config) -> MyResult<()> {
    for filename in config.files {
        match open(&filename) {
            Err(err) => eprintln!("Failed to open {filename}: {err}"),
            Ok(file) => {
                let mut last_num = 0;
                for (line_num, line) in file.lines().enumerate() {
                    let line = line?;
                    if config.number_lines {
                        println!("{:6}\t{}", line_num + 1, line);
                        continue;
                    }

                    if config.number_nonblank_lines {
                        if line.is_empty() {
                            println!();
                            continue;
                        }
                        last_num += 1;
                        println!("{:6}\t{}", last_num, line);
                        continue;
                    }

                    println!("{line}");
                }
            }
        }
    }
    Ok(())
}
