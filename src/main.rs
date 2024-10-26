use colored::*;
use std::process;

mod col;
mod cli;

fn main() {
    match cli::run() {
        Ok(true) => process::exit(0),
        Ok(false) => process::exit(1),
        Err(e) => {
            eprintln!("{}: {}", "Error".bright_red(), e);
            std::process::exit(127);
        }
    }
}
