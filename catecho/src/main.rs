use clap::Parser;

#[derive(Parser, Debug)]
#[command(author = "Vermee81")]
#[command(version, about = "An echo-like command adds cat meows")]
struct Args {
    #[arg(required = true, help = "Input text")]
    text: Vec<String>,
    #[arg(short = 'n', help = "Do not print newline")]
    no_newline: bool,
}

fn main() {
    let args = Args::parse();
    let text = args.text;
    let no_newline = args.no_newline;
    print!("{}{}{}", text.join(" ")," MEOW",if no_newline {""} else {"\n"} );
}
