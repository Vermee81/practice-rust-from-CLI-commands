fn main() {
    if let Err(e) = concat::get_args().and_then(concat::run) {
        eprintln!("{}", e);
        std::process::exit(1);
    }
}
