use crate::input_validator::Validator;

use std::env;
use std::path::Path;
use std::io::{self, Error, ErrorKind, Write, stdout};
use std::fs::File;
use std::process::{Command as ProcCommand, Stdio}; 
use colored::Colorize;

enum Command {
    CD,
    LS,
    MKDIR,
    PLUSPLUS,
    MINUSMINUS,
    KILL,
    UNKNOWN,
    PWD,
    HELP,
    DIRCONTENT
}

/// Handles various commands and executes corresponding actions.
pub fn handleCommand(command: &str, mut args: std::str::SplitWhitespace) -> Result<(), io::Error> {

    let mut previous_stdout: Option<std::process::ChildStdout> = None;

    match get_command_enum(command) {
        Command::CD => {
            let new_dir = args.clone().next().unwrap_or("/");
            let root = Path::new(new_dir);
            env::set_current_dir(&root).map_err(|e| {
                Error::new(e.kind(), format!("cd: {}: {}", new_dir, e))
            })
        }
        Command::LS => {
            // Check for pipe
            if peek_next(&mut args) == Some("|".to_string()) {
                args.next(); // consume the "|"
                if let Some(next_cmd) = args.next() {
                    // First command: ls
                    let ls_child = ProcCommand::new("ls")
                        .stdout(Stdio::piped())
                        .spawn()
                        .map_err(|_| Error::new(ErrorKind::NotFound, "ls not found"))?;

                    // Second command: wc or other command
                    let mut wc_child = ProcCommand::new(next_cmd)
                        .stdin(Stdio::from(ls_child.stdout.unwrap()))
                        .stdout(Stdio::inherit())
                        .spawn()
                        .map_err(|_| Error::new(ErrorKind::NotFound, format!("{} not found", next_cmd)))?;

                    wc_child.wait().unwrap();
                    return Ok(());
                }
            }

            // Normal ls without piping
            let path = args.clone().next().unwrap_or(".");
            print_ls(&path);
            Ok(())
        }
        Command::MKDIR => {

            let dir_name = match args.next() {
                Some(name) => name,
                None => {
                    println!("{}", "Error: Missing directory name for mkdir command".red());
                    return Ok(());  
                }
            };

            if let Err(e) = std::fs::create_dir_all(dir_name) {
                println!("Failed to create directory: {}", e);
            }
            Ok(())
        }
        Command::PLUSPLUS => {

            let file_name = match args.next() {
                Some(name) => name,
                None => {
                    println!("{}", "Error: Missing file name argument for ++ command".red());
                    return Ok(());  
                }
            };

            let mut validator = Validator::new();
            validator.add_rule(("file_name", Box::new(|input: &str| !input.is_empty())));
            validator.add_rule(("file_does_not_exist", Box::new(|input: &str| !Path::new(input).exists())));

            if !validator.validate(file_name) {
                println!("{}", format!("Invalid input: {}", file_name).red());
                return Ok(());  
            }

            File::create(file_name).map_err(|e| {
                println!("Failed to create file: {}", e);
                std::io::Error::new(e.kind(), format!("Failed to create file: {}", e))
            })?;

            println!("{}", format!("\nFile created successfully!\n").green());
            Ok(())
        }
        Command::MINUSMINUS => {

            let file_name = match args.next() {
                Some(name) => name,
                None => {
                    println!("{}", "Error: No file specified for -- command".red());
                    return Ok(());
                }
            };

            let dir = env::current_dir()?;
            let full_path = dir.join(file_name);

            if !full_path.exists() {
                println!("{}", format!("File not found: {}", file_name).red());
                return Ok(());
            }

            print!("{}", format!("\nAre you sure you want to delete {} (yes/no)?\n", file_name).red());

            let mut input = String::new();
            io::stdin().read_line(&mut input)?;

            if input.trim() == "yes" {
                std::fs::remove_file(&full_path)?;
                println!("{}", format!("\nFile deleted: {}\n", file_name).green());
            } else {
                println!("Deletion canceled.");
            }

            Ok(())
        }
        Command::KILL => std::process::exit(0),
        Command::PWD => {
            println!("{}", env::current_dir().unwrap().display());
            Ok(())
        },
        Command::HELP => {
            print_help();
            Ok(())
        },
        Command::DIRCONTENT => {
            let new_dir = args.clone().next().unwrap_or("/");
            let root = Path::new(new_dir);
            get_dir_content(&root.display().to_string());
            Ok(())
        },
        Command::UNKNOWN => Err(Error::new(ErrorKind::NotFound, "Command not found")),
    }
}

/// Maps a given command string to its corresponding enum variant.
fn get_command_enum(command: &str) -> Command {
    match command {
        "cd" => Command::CD,
        "ls" => Command::LS,
        "mkdir" => Command::MKDIR,
        "++" => Command::PLUSPLUS,
        "--" => Command::MINUSMINUS,
        "pwd" => Command::PWD,
        "kill" => Command::KILL,
        "help" => Command::HELP,
        "dircontent" => Command::DIRCONTENT,
        _ => Command::UNKNOWN,
    }
}

/// Peek at the next argument in the iterator, without consuming it.
/// Useful for error checking without advancing the iterator.
fn peek_next(args: &mut std::str::SplitWhitespace) -> Option<String> {
    args.clone().next().map(|s| s.to_string())
}

/// Prints the contents of the directory specified by the given path.
///
/// # Arguments
///
/// * `path` - A string slice that holds the path of the directory to list.
///
/// This function reads the directory entries and prints each entry's file name
/// to the standard output. It assumes the directory exists and panics if there
/// is an error reading the directory or its entries.
fn print_ls(path: &str) {
    println!();
    let root = std::path::Path::new(path);
    match root.read_dir() {
        Ok(entries) => {
            for entry_res in entries {
                if let Ok(entry) = entry_res {
                    // Print file names without extra spaces, flush after each
                    print!("\t> {}\n", entry.file_name().to_string_lossy().trim_start());
                    stdout().flush().unwrap();
                }
            }
        },
        Err(e) => eprintln!("Failed to read directory {}: {}", path, e),
    }
    println!();
}

/// Prints a help message to the standard output.
///
/// This function prints a summary of the available commands and their
/// respective usage.
fn print_help() {

    println!("{}", "\n--------------------\n".blue());
            println!("{}", "Commands:\n".bold());

            println!("{}", "Usage:".yellow());
            println!("  cd [directory]");
            println!("  ls [directory]");
            println!("  mkdir [directory]");
            println!("  ++ [file_name]");
            println!("  -- [file_name]");
            println!("  kill");
            println!("  pwd");
            println!("  dircontent [directory]");
            println!("  help");

            println!("{}", "\nFunctionality:".yellow());
            println!(
                "{}",
                "  cd      : Navigates to the specified directory.".italic()
            );
            println!(
                "{}",
                "  ls      : Displays the files and directories within the specified directory.".italic()
            );
            println!("{}", "  mkdir   : Creates a new directory with the given name.".italic());
            println!("{}", "  ++      : Creates a new file with the specified name.".italic());
            println!("{}", "  --      : Deletes the specified file.".italic());
            println!("{}", "  kill    : Terminates the shell session.".italic());
            println!(
                "{}",
                "  pwd     : Displays the path of the current working directory.".italic()
            );
            println!(
                "{}",
                "  dircontent : Lists the contents of the specified directory.".italic()
            );
            println!(
                "{}",
                "  help    : Provides a list of available commands and their descriptions.".italic()
            );

            println!("{}", "\n--------------------\n".blue());


}


/// Prints the contents of the specified directory.
///
/// This function reads the contents of the directory specified in the `path`
/// parameter and prints the names of the files and directories within it to the
/// standard output.
///
/// # Arguments
///
/// * `path`: The path to the directory to be read.
fn get_dir_content(path: &str) {
    // Read the contents of the specified directory
    let entries = match std::fs::read_dir(path) {
        Ok(entries) => entries,
        Err(e) => {
            eprintln!("Failed to read directory {}: {}", path, e);
            return;
        }
    };

    // Print the directory header
    println!("{}", "\n--------------------\n".blue());
    println!("{}", format!("Contents of {}:", path).bold());
    // Print the contents of the directory
    for entry in entries {
        match entry {
            Ok(entry) => println!("\t> {}", entry.path().display().to_string().replace("src/", "")),
            Err(e) => {
                eprintln!("Failed to read entry in {}: {}", path, e);
            }
        }
    }
    // Print the directory footer
    println!("{}", "\n--------------------\n".blue());
}
