use colored::*;

pub fn print_error(content: &str) {
    println!("[{}] {}", "Error".red().bold(), content)
}