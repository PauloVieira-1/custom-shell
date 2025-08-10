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
    PWD
}

/// Handles various commands and executes corresponding actions.
pub fn handleCommand(command: &str, mut args: std::str::SplitWhitespace) -> Result<(), io::Error> {
    // Will store output from a previous command for piping
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

                    // Second command: wc
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
            let dir_name = args.next().unwrap();
            std::fs::create_dir_all(dir_name).unwrap();
            Ok(())
        }
        Command::PLUSPLUS => {
            if args.clone().next().is_none() {
                return Err(Error::new(ErrorKind::InvalidInput, "No file specified"));
            }
            File::create(format!("{}", args.next().unwrap()))?;
            println!("{}", format!("File created Successfully!").green());
            Ok(())
        }
        Command::MINUSMINUS => {
            let file_name = match args.next() {
                Some(name) => name,
                None => return Err(Error::new(ErrorKind::InvalidInput, "No file specified")),
            };

            
            let dir = env::current_dir()?;
            let full_path = dir.join(file_name);
            
            if !full_path.exists() {
                println!("{}", format!("File not found: {}", file_name).red());
                return Ok(());
            }
            
            print!("{}", format!("Are you sure you want to delete {} (yes/no)?\n", file_name).red());

            let mut input = String::new();
            io::stdin().read_line(&mut input)?;

            if input.trim() == "yes" {
                std::fs::remove_file(&full_path)?;
                println!("File deleted: {}", file_name);
            } else {
                println!("Deletion canceled.");
            }

            Ok(())
        }
        Command::KILL => std::process::exit(0),
        Command::PWD => {println!("{}", env::current_dir().unwrap().display()); Ok(())},
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
        _ => Command::UNKNOWN,
    }
}


/// Peek at the next argument in the iterator, without consuming it.
/// Useful for error checking without advancing the iterator.
///
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
    print!("\n");
    let root = Path::new(path);
    for entry in root.read_dir().unwrap() {
        let entry = entry.unwrap();
        println!("\t> {}", entry.file_name().to_string_lossy());
    }
    print!("\n");
}
