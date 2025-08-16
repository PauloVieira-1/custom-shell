use serde::{Serialize, Deserialize};
use colored::{Colorize, Color as ColoredColor};
use crate::helpers::{update_config, get_home_dir};
use crate::command_handler::{get_color};

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
    // Get the first argument after the command
    let second_arg = match args.next() {
        Some(arg) => arg,
        None => {
            eprintln!("Missing customization option");
            return Ok(());
        }
    };

    let third_arg = args.next();

    if second_arg.to_string() == "help" {
        print_customization_options();
        return Ok(());
    }

    match CustomizationOptions::from_str(second_arg) {
        Some(CustomizationOptions::TextColor) => {
            
            for config in config.iter_mut() {
                if config.option == CustomizationOptions::TextColor {
                    config.value = Some(third_arg.unwrap_or("default").to_string());
                }
            }
            
            let config_path = format!("{}/.mysh_config", get_home_dir());
            update_config(config, &config_path)?;

            let color = get_color(CustomizationOptions::TextColor, config);
            let formated = format!("Changed Text Color to {}", color.make_str().bold());
            print_message(&formated, color);

        }
        Some(CustomizationOptions::BackgroundColor) => {
            println!("Change Background Color to {:?}", third_arg.unwrap_or("default"));
        }
        Some(CustomizationOptions::FontSize) => {
            println!("Change Font Size to {:?}", third_arg.unwrap_or("default"));
        }
        Some(CustomizationOptions::ErrorColor) => {
            println!("Change Error Color to {:?}", third_arg.unwrap_or("default"));

            for config in config.iter_mut() {
                if config.option == CustomizationOptions::ErrorColor {
                    config.value = Some(third_arg.unwrap_or("default").to_string());
                }
            }

            let config_path = format!("{}/.mysh_config", get_home_dir());
            update_config(config, &config_path)?;
        }
        Some(CustomizationOptions::PromptColor) => {
            println!("Change Prompt Color to {:?}", third_arg.unwrap_or("default"));
        }
        Some(CustomizationOptions::PromptText) => {
            println!("Change Prompt Text to {:?}", third_arg.unwrap_or("default"));
        }
        None => {
            eprintln!("Unknown customization option: {}", second_arg);
        }
    }

    Ok(())
}


/// Returns a vector containing all possible `CustomizationOptions`.
pub fn get_customization_options() -> Vec<Configuration> {
    // vec![
    //     CustomizationOptions::TextColor,
    //     CustomizationOptions::BackgroundColor,
    //     CustomizationOptions::FontSize,
    //     CustomizationOptions::ErrorColor,
    //     CustomizationOptions::PromptColor,
    //     CustomizationOptions::PromptText,
    // ]

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
}


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

pub fn change_config(config: &mut Configuration, value: &str) {
    config.value = Some(value.to_string());
}

