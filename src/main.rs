mod input_validator;
mod command_handler;

use command_handler::handleCommand;
use std::io::{stdin, stdout, Write, Result};
use std::process::Command;
use std::fs::File;

fn main() -> Result<()> {
    // create history file
    let mut history_file = File::create(format!("{}/.mysh_history", get_home_dir()))?;

    loop {
        print!("[<Prompt>] ");
        stdout().flush()?; 


        let mut input = String::new();
        if stdin().read_line(&mut input)? == 0 {
            break;
        }
        
        write_to_history(input.clone().trim().to_string(), &mut history_file)?;
        
        // validate input and split
        let mut parts = input.trim().split_whitespace();

        let Some(command) = parts.next() else {
            continue;
        };

        let args = parts;

        // handle command
        if let Err(e) = handleCommand(command, args.clone()) {
            println!("{}", e);
            continue;
        };

    }

    Ok(())
}

/// Returns the path to the user's home directory.
///
/// # Panics
/// Panics if the `HOME` environment variable is not set.
fn get_home_dir() -> String {
    let home_dir = std::env::var("HOME").unwrap();
    home_dir
}

/// Writes a line of input to the history file.
///
/// # Arguments
///
/// * `input`: The line of input to be written to the history file.
/// * `history_file`: The file to write the input to.
///
/// # Errors
/// This function will panic if there is an error writing to the file.
fn write_to_history(input: String, history_file: &mut File) -> Result<()> {
    history_file.write_all(input.as_bytes())?;
    history_file.write_all(b"\n")?;
    Ok(())
}

