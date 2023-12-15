use std::process::exit;

pub fn error(message: String) {
    eprintln!("{message}");
    exit(1);
}
