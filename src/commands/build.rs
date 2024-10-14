use std::process::Command;
use std::path::{Path, PathBuf};
use std::ffi::OsStr;
use crate::messages::error_message;
use crate::messages::success_message;

pub fn build(filepath: &str) {
    let path = Path::new(filepath);

    if !path.exists() {
        error_message::print_error(&format!("This file does not exist: {}", filepath));
        return;
    }

    let parent_folder = match path.parent() {
        Some(parent) => parent.file_name().unwrap_or_else(|| OsStr::new("unknown")).to_string_lossy().to_string(),
        None => {
            error_message::print_error("Could not determine the parent folder.");
            return;
        }
    };

    let output_file = format!("{}/{}.exe", path.parent().unwrap().display(), parent_folder);

    let output = Command::new("rustc")
        .arg(filepath)
        .arg("-o")
        .arg(&output_file)
        .output()
        .expect("failed to execute process");

    if output.status.success() {
        success_message::print_success(&format!("Successfully compiled the file. Executable file: {}", &output_file));
    } else {
        error_message::print_error("Error while compiling the file.");
        error_message::print_error(&format!("Stderr: {}", String::from_utf8_lossy(&output.stderr)));
    }
}
