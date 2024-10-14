use colored::*;

pub fn help() {
  println!("{} - The {} package manager for Rust\n", "rpack".red().bold(), "advanced".red().bold());
  println!("{} rpack [COMMAND]\n", "Usage:".cyan().bold());
  println!("{}
help/h      Displays the help menu.
build/b     Builds a rust source file.
  ", "Commands:".cyan().bold())
}