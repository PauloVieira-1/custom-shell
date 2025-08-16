use serde::{Serialize, Deserialize};
use colored::{Colorize, Color as ColoredColor};
use crate::helpers::{update_config, get_home_dir};
use crate::command_handler::{get_color};
use std::io::{Write, stdout};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Color {
    Red,
    Green,
    Blue,
    Yellow,
    Magenta,
    Cyan,
    White,
    Black,
}

impl Color {
    pub fn from_str(s: &str) -> Option<Self> { // self because Color is a struct
        match s {
            "Red" => Some(Color::Red),
            "Green" => Some(Color::Green),
            "Blue" => Some(Color::Blue),
            "Yellow" => Some(Color::Yellow),
            "Magenta" => Some(Color::Magenta),
            "Cyan" => Some(Color::Cyan),
            "White" => Some(Color::White),
            "Black" => Some(Color::Black),
            _ => None,
        }
    }

    pub fn make_str(self) -> &'static str {
        match self {
            Color::Red => "Red",
            Color::Green => "Green",
            Color::Blue => "Blue",
            Color::Yellow => "Yellow",
            Color::Magenta => "Magenta",
            Color::Cyan => "Cyan",
            Color::White => "White",
            Color::Black => "Black",
        }
    }

    pub fn get_color_list() -> Vec<Color> {
        vec![Color::Red, Color::Green, Color::Blue, Color::Yellow, Color::Magenta, Color::Cyan, Color::White, Color::Black]
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CustomizationOptions {
    TextColor,
    BackgroundColor,
    FontSize,
    ErrorColor,
    PromptColor,
    PromptText,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Configuration {
    pub option: CustomizationOptions,
    pub value: Option<String>,
}

impl CustomizationOptions {
    /// Returns the byte representation of the customization option.
    pub fn as_bytes(&self) -> &'static [u8] {
        match self {
            CustomizationOptions::TextColor => b"Text_Color",
            CustomizationOptions::BackgroundColor => b"Background_Color",
            CustomizationOptions::FontSize => b"Font_Size",
            CustomizationOptions::ErrorColor => b"Error_Color",
            CustomizationOptions::PromptColor => b"Prompt_Color",
            CustomizationOptions::PromptText => b"Prompt_Text", // fixed case consistency
        }
    }

    /// Returns the string representation of the customization option.
    pub fn as_str(&self) -> &'static str {
        std::str::from_utf8(self.as_bytes()).unwrap()
    }

    /// Attempts to parse a &str into a `CustomizationOptions` variant.
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "Text_Color" => Some(CustomizationOptions::TextColor),
            "Background_Color" => Some(CustomizationOptions::BackgroundColor),
            "Font_Size" => Some(CustomizationOptions::FontSize),
            "Error_Color" => Some(CustomizationOptions::ErrorColor),
            "Prompt_Color" => Some(CustomizationOptions::PromptColor),
            "Prompt_Text" => Some(CustomizationOptions::PromptText),
            _ => None,
        }
    }
}

/// Handles the `customize` command safely.
pub fn handle_customize(args: &mut std::str::SplitWhitespace, config: &mut Vec<Configuration>) -> Result<(), std::io::Error> {
    let color = get_color(CustomizationOptions::TextColor, config);
    let error_color = get_color(CustomizationOptions::ErrorColor, config);
    // Get the first argument after the command
    let second_arg = match args.next() {
        Some(arg) => arg,
        None => {
            print_message("Error: Missing second argument for customize command", color);
            return Ok(());
        }
    };

    let third_arg = args.next();

    if second_arg.to_string() == "--help" {
        print_customization_options();
        return Ok(());
    }

    match CustomizationOptions::from_str(second_arg) {
        Some(CustomizationOptions::TextColor) => {change_text_color(config, third_arg, CustomizationOptions::TextColor);}
        Some(CustomizationOptions::BackgroundColor) => {
            println!("Change Background Color to {:?}", third_arg.unwrap_or("default"));
        }
        Some(CustomizationOptions::FontSize) => {
            println!("Change Font Size to {:?}", third_arg.unwrap_or("default"));
        }
        Some(CustomizationOptions::ErrorColor) => {change_text_color(config, third_arg, CustomizationOptions::ErrorColor);}
        Some(CustomizationOptions::PromptColor) => {change_text_color(config, third_arg, CustomizationOptions::PromptColor);}
        Some(CustomizationOptions::PromptText) => {change_prompt_text(config, third_arg, CustomizationOptions::PromptText);} 
        None => {print_message("Error: Invalid customization option", error_color);}
    }

    Ok(())
}


/// Change the text color of the given `text_type` in the `config` vector to the given `color_name`.
/// 
/// If `color_name` is `None`, the text color is changed to the default color.
/// 
/// The `config` vector is updated and saved to the `.mysh_config` file.
/// 
/// Returns `Ok(())` if the text color was changed successfully, or an `Err` if there was an error.
pub fn change_text_color(config: &mut Vec<Configuration>, third_arg: Option<&str>, text_type: CustomizationOptions) -> Result<(), std::io::Error> {
    let color_name = third_arg.unwrap_or("default");
    let color = Color::from_str(color_name).unwrap_or(Color::Red);

    for config in config.iter_mut() {
        if config.option == text_type {
            config.value = Some(color_name.to_string());
        }
    }

    let config_path = format!("{}/.mysh_config", get_home_dir());
    update_config(config, &config_path)?;

    let formated = format!("Changed {} Color to {}", text_type.as_str(), color.make_str().bold());
    print_message(&formated, color);
    Ok(())
}


pub fn change_prompt_text(config: &mut Vec<Configuration>, third_arg: Option<&str>, text_type: CustomizationOptions) -> Result<(), std::io::Error> {
    let text = third_arg.unwrap_or("Prompt");
    let color = get_color(CustomizationOptions::TextColor, config);

    for config in config.iter_mut() {
        if config.option == text_type {
            config.value = Some(text.to_string());
        }
    }

    let config_path = format!("{}/.mysh_config", get_home_dir());
    update_config(config, &config_path)?;

    let formated = format!("Changed prompt to {}", text.bold());
    print_message(&formated, color);
    Ok(())
}


/// Returns a vector containing all possible `CustomizationOptions`.
pub fn get_customization_options() -> Vec<Configuration> {

    let configs_vector = vec![
        Configuration { option: CustomizationOptions::TextColor, value: None },
        Configuration { option: CustomizationOptions::BackgroundColor, value: None },
        Configuration { option: CustomizationOptions::FontSize, value: None },
        Configuration { option: CustomizationOptions::ErrorColor, value: None },
        Configuration { option: CustomizationOptions::PromptColor, value: None },
        Configuration { option: CustomizationOptions::PromptText, value: None },
    ];
    configs_vector
}


pub fn print_customization_options() {
    println!("\n+------------------------------------+");
    println!("| Available Customization Options:   |");
    println!("+------------------------------------+");

    let configs_vector = get_customization_options();

    for config in configs_vector {
        let option = config.option;
        let value = config.value;

        println!("| {:<20} | {:<10}  |", option.as_str(), value.unwrap_or("default".to_string()));
    }

    println!("+------------------------------------+\n");

    println!("\n+----------------------+");
    println!("| Available Colors :   |");
    println!("+----------------------+");

    for color in Color::get_color_list() {
        println!("| {:<20} |", color.make_str());
    }

    println!("+----------------------+\n");

}


/// Prints the shell's prompt to the standard output with the given text and color.
///
/// # Arguments
///
/// * `text` - The text to be displayed in the prompt.
/// * `color` - The color to be applied to the prompt.
///
/// # Returns
///
/// Returns a `Result` indicating whether the prompt was printed successfully or not.
pub fn print_prompt(text: &str, color: Color) -> Result<(), std::io::Error> {
    let formatted = format!("[<{}>] ", text); // note the space for input
    match color {
        Color::Red => print!("{}", formatted.red()),
        Color::Green => print!("{}", formatted.green()),
        Color::Yellow => print!("{}", formatted.yellow()),
        Color::Blue => print!("{}", formatted.blue()),
        Color::Magenta => print!("{}", formatted.magenta()),
        Color::Cyan => print!("{}", formatted.cyan()),
        Color::White => print!("{}", formatted.white()),
        _ => print!("{}", formatted),
    }
    stdout().flush()?; // ensures the prompt appears immediately
    Ok(())
}

/// Prints the given `message` with the given `color`.
///
/// # Arguments
///
/// * `message`: A string representing the message to be printed.
/// * `color`: A `Color` enum representing the color to be applied to the message.
pub fn print_message(message: &str, color: Color) {
    match color {
        Color::Red => println!("{}", message.red()),
        Color::Green => println!("{}", message.green()),
        Color::Yellow => println!("{}", message.yellow()),
        Color::Blue => println!("{}", message.blue()),
        Color::Magenta => println!("{}", message.magenta()),
        Color::Cyan => println!("{}", message.cyan()),
        Color::White => println!("{}", message.white()),
        _ => println!("{}", message),
    }
}


/// Updates the value of the given `Configuration` struct with the given string.
///
/// # Arguments
///
/// * `config`: A mutable reference to a `Configuration` struct.
/// * `value`: A string representing the new value to assign to the `Configuration` struct.
pub fn change_config(config: &mut Configuration, value: &str) {
    config.value = Some(value.to_string());
}

