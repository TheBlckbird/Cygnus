use std::process::exit;

pub fn print_error(message: String) {
    eprintln!("{message}\n",);
    exit(1);
}
