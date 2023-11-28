pub const RED: &'static str = "\x1b[31m";
pub const GREEN: &'static str = "\x1b[32m";
pub const YELLOW: &'static str = "\x1b[33m";
pub const RESET: &'static str = "\x1b[0m";
pub const BOLD: &'static str = "\x1b[1m";

pub fn warning(message: &str) {
    println!("{}{}{}", YELLOW, message, RESET);
}


pub fn error(message: &str) {
    println!("{}{}{}", RED, message, RESET);
}


pub fn success(message: &str) {
    println!("{}{}{}", GREEN, message, RESET);
}
