fn main() {
    if let Err(e) = concat::run() {
        eprintln!("{}", e);
        std::process::exit(1);
    }
}
