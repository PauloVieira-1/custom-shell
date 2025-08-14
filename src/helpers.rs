use crate::customization_handler::{get_customization_options, CustomizationOptions};

use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, Result, Write, stdout};
use std::path::Path;
use crossterm::{
    cursor::MoveToColumn,
    execute,
    terminal::{Clear, ClearType},
};


/// Returns the path to the user's home directory.
///
/// # Panics
/// Panics if the `HOME` environment variable is not set.
pub fn get_home_dir() -> String {
    std::env::var("HOME").expect("HOME environment variable not set")
}


/// Initializes the shell's history file.
///
/// The history file is a file that stores the shell's history of commands. It is
/// created if it does not already exist. The function returns a handle to the
/// file.

pub fn initialize_history_file() -> File {
    let history_path = format!("{}/.mysh_history", get_home_dir());
    if !Path::new(&history_path).exists() {
        File::create(&history_path).unwrap();
    }

    OpenOptions::new()
        .read(true)
        .append(true) // so new history lines are added, not overwrite
        .open(&history_path)
        .unwrap()

}


/// Initializes the shell's configuration file.
///
/// The configuration file is a file that stores the shell's configuration.
/// It is created if it does not already exist. The function returns a handle
/// to the file.
pub fn initialize_config_file() -> File {
    let config_path = format!("{}/.mysh_config", get_home_dir());
    if !Path::new(&config_path).exists() {
        File::create(&config_path).unwrap();
    }

    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .open(&config_path)
        .unwrap();

    let configs_vector: Vec<CustomizationOptions> = get_customization_options();

    for config in configs_vector {
        file.write_all(config.as_bytes()).unwrap();
        file.write_all(b"\n").unwrap();
    }
    
    file
    
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
pub fn write_to_history(input: String, history_file: &mut File) -> Result<()> {
    if input.trim().is_empty() {
        return Ok(());
    }
    history_file.write_all(input.as_bytes())?;
    history_file.write_all(b"\n")?;
    Ok(())
}

/// Reads the last line from the shell's history file.
///
/// # Returns
/// A string containing the last line in the history file. If the file is empty,
/// an empty string is returned.
pub fn get_prev_command(command_list: &mut Vec<String>, last_index: &mut usize) -> String {
    let current_index = *last_index;
    if current_index == 0 {
        String::new()
    } else {
        *last_index -= 1;
        command_list[current_index - 1].clone()
    }
}

/// Retrieves the next command from the command history.
///
/// This function updates the provided index to point to the next command
/// in the history list, if there is one, and returns that command. If the
/// index is already at the last command in the list, it returns an empty
/// string.
///
/// # Arguments
///
/// * `commands`: A mutable reference to the vector containing the command history.
/// * `index`: A mutable reference to the current index in the command history.
///
/// # Returns
/// A string containing the next command in the history. Returns an empty string
/// if there are no more commands to retrieve.

pub fn get_next_command(commands: &mut Vec<String>, index: &mut usize) -> String {
    if commands.is_empty() {
        String::new()
    } 
    else if *index >= commands.len() - 1 {
        String::new()
    } else {
        *index += 1;
        commands[*index].clone()
    }
}

/// Reads the contents of the given file into a vector of strings.
///
/// # Arguments
///
/// * `file`: A mutable reference to the file to read from.
///
/// # Returns
/// A vector of strings, each of which is a line from the file.
pub fn read_history(file : &File) -> Vec<String> {
    let mut result = Vec::new();
    let reader = BufReader::new(file);

    for line in reader.lines() {
        result.push(line.unwrap());
    }
    result
}

/// Clears the current line in the terminal.
///
/// This function is used to erase the current command line input. It does so by
/// moving the cursor to the beginning of the current line and clearing the line.
/// After clearing the line, the cursor is moved back to the beginning of the line.
///
/// # Errors
///
/// If there is an error clearing the line or moving the cursor, an error is
/// returned.
pub fn clear_current_line() -> Result<()> {
    let mut stdout = stdout();
    execute!(
        stdout,
        Clear(ClearType::CurrentLine),
        MoveToColumn(0)
    )?;
    stdout.flush()?;
    Ok(())
}

/// Prints the shell's prompt to the standard output.
///
/// # Errors
/// Returns an error if there is a problem printing to the standard output.
pub fn print_prompt() -> Result<()> {
    print!("[<Prompt>] ");
    stdout().flush()?;
    Ok(())
}


