use crate::input_validator::Validator;
use crate::helpers::{get_home_dir, initialize_history_file};
use crate::customization_handler::{handle_customize, print_message, Configuration, CustomizationOptions, Color};


use std::env;
use std::path::Path;
use std::io::{self, Error, ErrorKind, Write, stdout};
use std::fs::File;
use std::process::{Command as ProcCommand, Stdio}; 
use colored::{Colorize, Color as ColoredColor};
use std::fs::OpenOptions;

pub enum Command {
    CD,
    LS,
    MKDIR,
    PLUSPLUS,
    MINUSMINUS,
    KILL,
    UNKNOWN,
    PWD,
    HELP,
    DIRCONTENT,
    CLEAR,
    CUSTOMIZE
}

/// Handles various commands and executes corresponding actions.
pub fn execute_command(command: &str, mut args: std::str::SplitWhitespace, current_config: &mut Vec<Configuration>) -> Result<(), io::Error> {

    // Helper to wrap functions that return () into Result<(), Error>
    let mut args = args;
    let mut run = |f: fn(&mut std::str::SplitWhitespace, &mut Vec<Configuration>) -> Result<(), Error>| -> Result<(), Error> {
    f(&mut args, current_config)
    }; // this function is a closure that captures the args variable and passes it to the function

    let command = get_command_enum(command);

    match command {
        Command::CD => run(handle_current_dir),
        Command::LS => {run(list_dir); Ok(())},
        Command::MKDIR => run(make_dir),
        Command::PLUSPLUS => run(make_file),
        Command::MINUSMINUS => run(remove_file),
        Command::KILL => std::process::exit(0),
        Command::PWD => {
            let dir = std::env::current_dir()?;
            let color = get_color(CustomizationOptions::TextColor, current_config);
            print_message(&format!("{}", dir.display()), color);
            Ok(())
        }
        Command::HELP => { print_help(); Ok(()) },
        Command::DIRCONTENT => run(handle_dircontent),
        Command::CLEAR => { let _ = clear_history(); Ok(()) },
        Command::CUSTOMIZE => run(handle_customize),
        Command::UNKNOWN => {
            let unknown_command = args;
            let color = get_color(CustomizationOptions::ErrorColor, current_config);
            print_message("Unknown command", color);
            Ok(())
        },


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
        "clear" => Command::CLEAR,
        "customize" => Command::CUSTOMIZE,
        _ => Command::UNKNOWN,
    }
}


    /// Changes the current directory to the given argument.
    ///
    /// If no argument is given, the current directory is not changed.
    ///
    /// # Errors
    ///
    /// If the specified directory does not exist, an error is returned.
fn handle_current_dir(args: &mut std::str::SplitWhitespace, current_config: &mut Vec<Configuration>) -> Result<(), io::Error> {
    let new_dir = args.clone().next().unwrap_or("/");
    let root = Path::new(new_dir);
    env::set_current_dir(&root).map_err(|e| {
        let color = get_config_value(CustomizationOptions::ErrorColor, current_config)
                    .and_then(|color_str| Color::from_str(&color_str))
                    .unwrap_or(Color::Red);
        print_message(&format!("Failed to change directory: {}", e), color);
        e
    })
}

    /// Execute ls command with optional piping to another command.
    ///
    /// If the first argument is a pipe ("|"), it will be interpreted as a pipe
    /// command. In this case, the second argument will be executed with the
    /// output of the first command as its standard input.
    ///
    /// If there is no pipe argument, the command will be interpreted as a normal
    /// ls command.
    ///
    /// # Errors
    ///
    /// If the command is not found or there is another error executing the
    /// command, an error is returned.
fn list_dir(args: &mut std::str::SplitWhitespace, _config: &mut Vec<Configuration>) -> Result<(), Error> {
    // Normal & pipe handling
    if peek_next(args) == Some("|".to_string()) {
        args.next(); // consume "|"
        if let Some(next_cmd) = args.next() {
            let ls_child = ProcCommand::new("ls")
                .stdout(Stdio::piped())
                .spawn()
                .map_err(|_| Error::new(ErrorKind::NotFound, "ls not found"))?;

            let stdout = ls_child.stdout.ok_or_else(|| Error::new(ErrorKind::Other, "Failed to capture ls stdout"))?;

            let mut wc_child = ProcCommand::new(next_cmd)
                .stdin(Stdio::from(stdout))
                .stdout(Stdio::inherit())
                .spawn()
                .map_err(|_| Error::new(ErrorKind::NotFound, format!("{} not found", next_cmd)))?;

            wc_child.wait().map_err(|e| Error::new(ErrorKind::Other, e))?;
            return Ok(());
        }
    }

    // Normal ls without piping
    let path = args.next().unwrap_or(".");
    print_ls(path, _config);
    Ok(())
}


    /// Creates a new directory with the given name.
    ///
    /// This function takes one argument which is the name of the directory to be
    /// created. If the argument is not given, an error is returned.
    ///
    /// # Errors
    ///
    /// If the directory already exists, or if there is an error creating the
    /// directory, an error is returned.
fn make_dir(args: &mut std::str::SplitWhitespace, _config: &mut Vec<Configuration>) -> Result<(), Error> {
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

    /// Creates a new file with the given name.
    ///
    /// This function takes one argument which is the name of the file to be
    /// created. If the argument is not given, an error is returned.
    ///
    /// # Errors
    ///
    /// If the file already exists or if there is an error creating the file,
    /// an error is returned.
fn make_file(args: &mut std::str::SplitWhitespace, _config: &mut Vec<Configuration>) -> Result<(), Error> {
    let file_name = match args.next() {
                Some(name) => name,
                None => {
                    let color = get_color(CustomizationOptions::ErrorColor, _config);
                    print_message("Error: Missing file name argument for ++ command", color);
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

    /// Deletes a file with the given name.
    ///
    /// This function takes one argument which is the name of the file to be
    /// deleted. If the argument is not given, an error is returned.
    ///
    /// Before deleting the file, the function will prompt the user to confirm
    /// the deletion. If the user types 'yes', the file will be deleted.
    /// Otherwise, the deletion will be canceled.
    ///
    /// # Errors
    ///
    /// If the file does not exist or if there is an error deleting the file,
    /// an error is returned.
fn remove_file(args: &mut std::str::SplitWhitespace, _config: &mut Vec<Configuration>) -> Result<(), Error> {
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

    /// Lists the contents of the directory specified by the given path.
    ///
    /// If no argument is given, the current directory is used.
    ///
    /// # Errors
    ///
    /// If there is an error reading the directory or its entries, an error is
    /// returned.
fn handle_dircontent(args: &mut std::str::SplitWhitespace, _config: &mut Vec<Configuration>) -> Result<(), Error> {
    let new_dir = args.clone().next().unwrap_or("/");
    let root = Path::new(new_dir);
    get_dir_content(&root.display().to_string());
    Ok(())
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
fn print_ls(path: &str, _config: &mut Vec<Configuration>) {
    println!();
    let root = std::path::Path::new(path);
    match root.read_dir() {
        Ok(entries) => {
            for entry_res in entries {
                if let Ok(entry) = entry_res {
                    let color = get_color(CustomizationOptions::TextColor, _config);
                    let file_name = format!("\t> {}", entry.file_name().to_string_lossy().trim_start());
                    print_message(&file_name, color);
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

/// Clears the contents of the given file.
///
/// This function truncates the length of the file to zero, effectively clearing
/// its contents.
///
/// # Arguments
///
/// * `file`: The file to clear.
///
/// # Errors
///
/// If there is an error opening the file or setting its length, an error is
/// returned.
fn clear_history() -> Result<(), std::io::Error> {

    let history_path = format!("{}/.mysh_history", get_home_dir());
    std::fs::remove_file(history_path).unwrap();
    
    let history_file = initialize_history_file();
    history_file.set_len(0);

    Ok(())
}

/// Returns the value of the given configuration key from the given configuration vector.
///
/// # Arguments
///
/// * `key`: The configuration key to search for.
/// * `configs_vector`: The vector of `Configuration` structs to search through.
///
/// # Returns
///
/// An `Option<String>` containing the value of the given configuration key, or `None` if the key is not found.
fn get_config_value(key: CustomizationOptions, configs_vector: &mut Vec<Configuration>) -> Option<String> {
    for config in configs_vector {
        if config.option == key {
            return config.value.clone();
        }
    }
    None
}

/// Returns the color value associated with the given configuration key from the given configuration vector.
///
/// If the configuration key is not found or the value is not a valid color, returns `Color::Red`.
///
/// # Arguments
///
/// * `option`: The configuration key to search for.
/// * `configs_vector`: The vector of `Configuration` structs to search through.
///
/// # Returns
///
/// The color value associated with the given configuration key, or `Color::Red` if the key is not found or the value is not a valid color.
pub fn get_color(option: CustomizationOptions, configs_vector: &mut Vec<Configuration>) -> Color {
    let value = get_config_value(option, configs_vector).and_then(|color_str| Color::from_str(&color_str));
    value.unwrap_or(Color::Red)
}
