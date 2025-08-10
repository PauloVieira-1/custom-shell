use crossterm::{
    event::{read, Event, KeyCode},
    terminal::{enable_raw_mode, disable_raw_mode, Clear, ClearType},
    cursor::MoveToColumn,
    execute,
};
use std::io::{stdin, stdout, Write, Result};
use std::fs::File;
use std::io::{BufReader, BufRead};

mod input_validator;
mod command_handler;
use command_handler::handleCommand;

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

fn main() -> Result<()> {
   
    // create history file
    let mut history_file = File::create(format!("{}/.mysh_history", get_home_dir()))?;
    
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
                        if let Ok(prev_command) = get_prev_command() {
                            // erase current input line
                            for _ in 0..input.len() {
                                print!("\x08 \x08");
                            }
                            input.clear();
                            input = prev_command;
                            print!("{}", input);
                            stdout().flush()?;
                        }
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
        write_to_history(input.clone(), &mut history_file)?;

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
    use std::io::Write;
    history_file.write_all(input.as_bytes())?;
    history_file.write_all(b"\n")?;
    Ok(())
}

/// Reads the last line from the shell's history file.
///
/// # Returns
/// A string containing the last line in the history file. If the file is empty,
/// an empty string is returned.
fn get_prev_command() -> Result<String> {
    let file = File::open(format!("{}/.mysh_history", get_home_dir()))?;
    let reader = BufReader::new(file);
    let lines: Vec<String> = reader
        .lines()
        .collect::<std::result::Result<Vec<String>, std::io::Error>>()?;
    Ok(lines.last().cloned().unwrap_or_default())
}
