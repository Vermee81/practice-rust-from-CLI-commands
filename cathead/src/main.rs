fn main() {
    if let Err(e) = cathead::get_args().and_then(cathead::run) {
        eprintln!("{}", e);
        std::process::exit(1)
    }
}
