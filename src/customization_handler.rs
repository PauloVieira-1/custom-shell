use serde::{Serialize, Deserialize};
use colored::{Colorize, Color as ColoredColor};

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
    Gray,
    LightRed,
    LightGreen,
    LightBlue,
    LightYellow,
    LightMagenta,
    LightCyan,
    LightGray,
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
pub fn handle_customize(args: &mut std::str::SplitWhitespace, _config: &mut Vec<Configuration>) -> Result<(), std::io::Error> {
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
            println!("Change Text Color to {:?}", third_arg.unwrap_or("default"));
        }
        Some(CustomizationOptions::BackgroundColor) => {
            println!("Change Background Color to {:?}", third_arg.unwrap_or("default"));
        }
        Some(CustomizationOptions::FontSize) => {
            println!("Change Font Size to {:?}", third_arg.unwrap_or("default"));
        }
        Some(CustomizationOptions::ErrorColor) => {
            println!("Change Error Color to {:?}", third_arg.unwrap_or("default"));
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


pub fn print_message(message: &str, color: ColoredColor) {
    match color {
        ColoredColor::Red => println!("{}", message.red()),
        ColoredColor::Green => println!("{}", message.green()),
        ColoredColor::Yellow => println!("{}", message.yellow()),
        ColoredColor::Blue => println!("{}", message.blue()),
        ColoredColor::Magenta => println!("{}", message.magenta()),
        ColoredColor::Cyan => println!("{}", message.cyan()),
        ColoredColor::White => println!("{}", message.white()),
        _ => println!("{}", message),
    }
}

