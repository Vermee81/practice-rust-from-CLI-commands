use clap::Parser;

#[derive(Parser, Debug)]
#[command(author = "Vermee81")]
#[command(version, about = "An echo-like command adds cat meows")]
struct Args {
    #[arg(required = true, help = "Input text")]
    text: String,
}

fn main() {
    let args = Args::parse();
    println!("{:#?}", args);
}
