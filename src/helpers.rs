use crate::customization_handler::{get_customization_options, CustomizationOptions, Configuration};

use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, Result, Write, stdout};
use std::path::Path;
use crossterm::{
    cursor::MoveToColumn,
    execute,
    terminal::{Clear, ClearType},
};
use serde::{Serialize, Deserialize};


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
    if !check_path_exists(&history_path) {
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
    if !check_path_exists(&config_path) {
        let configs_vector: Vec<Configuration> = get_customization_options();
        let serialised = serde_json::to_string_pretty(&configs_vector).unwrap();
        File::create(&config_path).unwrap().write_all(serialised.as_bytes()).unwrap();
        return File::open(&config_path).unwrap();
    }

    OpenOptions::new()
        .read(true)
        .write(true)
        .open(&config_path).unwrap()
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


pub fn update_config(configs: &Vec<Configuration>, path: &str) -> Result<()> {
    // Serialize the whole vector as JSON
    let serialised = serde_json::to_string_pretty(configs)
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;

    // Overwrite the file with the new JSON
    let mut file = File::create(path)?;  
    file.write_all(serialised.as_bytes())?;
    Ok(())
}

/// Reads the contents of the given config file into a vector of `Configuration` structs.
///
/// # Arguments
///
/// * `config_file`: A mutable reference to the file to read from.
///
/// # Returns
///
/// A `Result` containing a vector of `Configuration` structs, or an error if there is an I/O or parse error.
pub fn read_config(config_file: &mut File) -> Result<Vec<Configuration>> {
    let reader = BufReader::new(config_file);
    let configs: Vec<Configuration> = serde_json::from_reader(reader)?;
    Ok(configs)
}

/// Returns a new `Configuration` vector with the given `option` added and all other
/// `Configuration` structs copied from the original `Configuration` vector.
///
/// # Arguments
///
/// * `option`: The customization option to add to the new `Configuration` vector.
/// * `value`: The value associated with the new customization option.
///
/// # Returns
/// A new `Configuration` vector with the given `option` added and all other
/// `Configuration` structs copied from the original `Configuration` vector.
pub fn add_option_to_config_vector(
    option: CustomizationOptions,
    value: String,
) -> Vec<Configuration> {
    let original_configs = get_customization_options();

    let mut updated_configs = Vec::new();

    for config in original_configs {
        updated_configs.push(if config.option == option {
            Configuration {
                option: config.option,
                value: Some(value.clone()),
            }
        } else {
            config
        });
    }

    updated_configs
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

/// Checks if a given path exists.
///
/// This function takes a `path` as a string and returns a boolean value indicating
/// whether the path exists or not. It uses the `std::path::Path` struct to check if
/// the given path exists on the file system.
///
/// # Arguments
///
/// * `path` - A string slice that holds the path to be checked.
///
/// # Returns
///
/// Returns `true` if the path exists, and `false` otherwise.
pub fn check_path_exists(path: &str) -> bool {
    Path::new(path).exists()
}

