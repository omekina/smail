const RED: &'static str = "\x1b[31m";
const GREEN: &'static str = "\x1b[32m";
const YELLOW: &'static str = "\x1b[33m";
const RESET: &'static str = "\x1b[0m";


pub fn warning(message: &str) {
    println!("{}{}{}", YELLOW, message, RESET);
}


pub fn error(message: &str) {
    println!("{}{}{}", RED, message, RESET);
}


pub fn success(message: &str) {
    println!("{}{}{}", GREEN, message, RESET);
}
