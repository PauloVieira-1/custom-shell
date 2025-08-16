use crossterm::{
    event::{read, Event, KeyCode},
    terminal::{enable_raw_mode, disable_raw_mode},
    cursor::MoveToColumn,
    execute,
};

use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, stdout, Write, Result};
use std::path::Path;

mod input_validator;

mod helpers;
use helpers::{
    initialize_config_file,
    initialize_history_file,
    read_history,
    get_prev_command,
    get_next_command,
    write_to_history,
    clear_current_line,
    read_config,
};

mod command_handler;
use command_handler::{execute_command, get_color, get_config_value};

mod customization_handler;
use customization_handler::{handle_customize, print_message, CustomizationOptions, print_prompt};


fn main() -> Result<()> {
   
    // create history file and config file 

    let mut history_file = initialize_history_file();
    let mut commands_list = read_history(&history_file);
    let mut index = commands_list.len();
    let mut config_file = initialize_config_file();
    let mut current_config = read_config(&mut config_file).unwrap();

    // create input buffer
    let mut input = String::new();
    let color = get_color(CustomizationOptions::TextColor, &mut current_config);

    // enable raw mode for capturing input key-by-key
    enable_raw_mode()?;

    loop {
        input.clear();
        let prompt_color = get_color(CustomizationOptions::PromptColor, &mut current_config);
        let prompt_text = get_config_value(CustomizationOptions::PromptText, &mut current_config).unwrap_or("PROMPT".to_string());
        print_prompt(&prompt_text, prompt_color)?;

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
                    KeyCode::Down => {
                        let next_command = get_next_command(&mut commands_list, &mut index);
                            // erase current input line
                            for _ in 0..input.len() {
                                print!("\x08 \x08");
                            }
                            input.clear();
                            input = next_command;
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

        if let Err(e) = execute_command(command, args.clone(), &mut current_config) {
            print_message(&e.to_string(), color);
            continue;
        }

        enable_raw_mode()?;
    }
}






// ===============================
//          Feature Plan
// ===============================

// Prompt Customization
// ------------------------------
// - Allow users to change prompt text and color (e.g., show username@hostname:cwd$).

// Tab Completion
// ------------------------------
// - Auto-complete command names and file paths when pressing Tab.

// Environment Variable Support
// ------------------------------
// - Expand $HOME, $PATH, $USER in commands.
// - Add set and unset commands.

// Alias System
// ------------------------------
// - Let users create shortcuts, e.g., alias ll='ls -la'.

// Piping & Redirection Enhancements
// ------------------------------
// - Support multiple pipes (cmd1 | cmd2 | cmd3).
// - Append redirection (>>).

// Wildcards & Globbing
// ------------------------------
// - Enable *.txt or file_?.rs matching.

// Background Job Control
// ------------------------------
// - Support jobs, fg, bg commands.
// - Show running jobs when exiting.

// Built-In Help System
// ------------------------------
// - Type help to show available commands and syntax.

// Subshell Execution
// ------------------------------
// - Support $(command) or backticks `command` for substitution.

