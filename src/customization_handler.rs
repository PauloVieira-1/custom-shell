#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CustomizationOptions {
    TextColor,
    BackgroundColor,
    FontSize,
    ErrorColor,
    PromptColor,
    PromptText,
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
pub fn handle_customize(args: &mut std::str::SplitWhitespace) -> Result<(), std::io::Error> {
    // Get the first argument after the command
    let second_arg = match args.next() {
        Some(arg) => arg,
        None => {
            eprintln!("Missing customization option");
            return Ok(());
        }
    };

    let third_arg = args.next();

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
pub fn get_customization_options() -> Vec<CustomizationOptions> {
    vec![
        CustomizationOptions::TextColor,
        CustomizationOptions::BackgroundColor,
        CustomizationOptions::FontSize,
        CustomizationOptions::ErrorColor,
        CustomizationOptions::PromptColor,
        CustomizationOptions::PromptText,
    ]
}
