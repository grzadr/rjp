fn main() {
    if let Err(e) = rjp::run_from_args() {
        eprintln!("{}", e);
        std::process::exit(1);
    }
}
