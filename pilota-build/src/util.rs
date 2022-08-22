pub fn error_abort(msg: String) -> ! {
    eprintln!("{}", msg);
    std::process::exit(1);
}
