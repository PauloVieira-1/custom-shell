use crossterm::{
    event::{read, Event, KeyCode},
    terminal::{enable_raw_mode, disable_raw_mode, Clear, ClearType},
    cursor::MoveToColumn,
    execute,
};
use std::io::{stdin, stdout, Write, Result};
use std::fs::File;
use std::io::{BufReader, BufRead};
use std::path::Path;
use std::fs::OpenOptions;

mod input_validator;
mod command_handler;
use command_handler::handleCommand;


fn main() -> Result<()> {
   
    // create history file and config file 

    let mut history_file = initialize_history_file();
    let mut commands_list = read_history(&history_file);
    let mut index = commands_list.len();
    let mut config_file = initialize_config_file();

    // create input buffer
    let mut input = String::new();

    // enable raw mode for capturing input key-by-key
    enable_raw_mode()?;

    loop {
        input.clear();
        print_prompt()?;

        loop {
            if let Event::Key(key) = read()? {
                match key.code {
                    KeyCode::Up => {
                        let prev_command = get_prev_command(&mut commands_list, &mut index); 
                            // erase current input line
                            for _ in 0..input.len() {
                                print!("\x08 \x08");
                            }
                            input.clear();
                            input = prev_command;
                            print!("{}", input);
                            stdout().flush()?;

                    
                        
                    }
                    KeyCode::Char(c) => {
                        input.push(c);
                        print!("{}", c);
                        stdout().flush()?;
                    }
                    KeyCode::Enter => {
                        print!("\n");
                        break;
                    }
                    KeyCode::Backspace => {
                        if input.pop().is_some() {
                            print!("\x08 \x08");
                            stdout().flush()?;
                        }
                    }
                    KeyCode::Esc => {
                        disable_raw_mode()?;
                        return Ok(());
                    }
                    _ => {}
                }
            }
        }

        // Write to history
        commands_list.push(input.clone());
        write_to_history(input.clone(), &mut history_file)?;
        index = commands_list.len();

        // Before running the command, disable raw mode and clear input line
        disable_raw_mode()?;

        // Clear the input line so output doesn't get mangled
        clear_current_line()?;

        // Parse command and arguments
        let mut parts = input.trim().split_whitespace();
        let Some(command) = parts.next() else {
            // Re-enable raw mode and prompt again
            enable_raw_mode()?;
            continue;
        };
        let args = parts;

        if let Err(e) = handleCommand(command, args.clone()) {
            println!("{}", e);
        }

        enable_raw_mode()?;
    }
}


/// Initializes the shell's history file.
///
/// The history file is a file that stores the shell's history of commands. It is
/// created if it does not already exist. The function returns a handle to the
/// file.

fn initialize_history_file() -> File {
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
fn initialize_config_file() -> File {
    let config_path = format!("{}/.mysh_config", get_home_dir());
    if !Path::new(&config_path).exists() {
        File::create(&config_path).unwrap();
    }

    OpenOptions::new()
        .read(true)
        .write(true)
        .open(&config_path)
        .unwrap()
    
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
fn clear_current_line() -> Result<()> {
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
fn print_prompt() -> Result<()> {
    print!("[<Prompt>] ");
    stdout().flush()?;
    Ok(())
}

/// Returns the path to the user's home directory.
///
/// # Panics
/// Panics if the `HOME` environment variable is not set.
fn get_home_dir() -> String {
    std::env::var("HOME").expect("HOME environment variable not set")
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
fn get_prev_command(command_list: &mut Vec<String>, last_index: &mut usize) -> String {
    let current_index = *last_index;
    if current_index == 0 {
        String::new()
    } else {
        *last_index -= 1;
        command_list[current_index - 1].clone()
    }
}

fn read_history(file : &File) -> Vec<String> {
    let mut result = Vec::new();
    let reader = BufReader::new(file);

    for line in reader.lines() {
        result.push(line.unwrap());
    }
    result
}
