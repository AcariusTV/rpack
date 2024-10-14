use colored::*;

pub fn print_success(content: &str) {
    println!("[{}] {}", "Success".green().bold(), content)
}