mod input_validator;
mod command_handler;

use command_handler::handleCommand;
use std::io::{stdin, stdout, Write, Result};
use std::process::Command;

fn main() -> Result<()> {
    loop {
        print!("[<Prompt>] ");
        stdout().flush()?; // remove unwrap before ?

        let mut input = String::new();
        if stdin().read_line(&mut input)? == 0 {
            break;
        }

        let mut parts = input.trim().split_whitespace();

        let Some(command) = parts.next() else {
            continue;
        };

        let args = parts;

        if let Err(e) = handleCommand(command, args.clone()) {
            println!("{}", e);
            continue;
        };

        // match Command::new(command).args(args).spawn() {
        //     Ok(mut child) => {
        //         if let Err(e) = child.wait() {
        //             println!("Failed to wait on process: {}", e);
        //         }
        //     }
        //     Err(_) => {
        //         println!("Command not found");
        //     }
        // }
    }

    Ok(())
}

// ===============================
//          Feature Plan
// ===============================

// Command History
// ------------------------------
// - Store and navigate through previous commands with ↑/↓ arrow keys.
// - Optional: Save history to ~/.mysh_history.

// Tab Completion
// ------------------------------
// - Auto-complete command names and file paths when pressing Tab.

// Prompt Customization
// ------------------------------
// - Allow users to change prompt text and color (e.g., show username@hostname:cwd$).

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

