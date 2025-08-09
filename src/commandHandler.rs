use std::env;
use std::path::Path;
use std::io::{self, Error, ErrorKind};
use std::fs::File;

pub fn handleCommand(command: &str, mut args: std::str::SplitWhitespace) -> Result<(), io::Error> {
    match command {
        "cd" => {
            let new_dir = args.clone().next().unwrap_or("/");
            let root = Path::new(new_dir);
            env::set_current_dir(&root).map_err(|e| {
                Error::new(e.kind(), format!("cd: {}: {}", new_dir, e))
            })
        },
        "ls" => {
            let path = args.clone().next().unwrap_or(".");
            let root = Path::new(path);
            print!("\n");
            for entry in root.read_dir()? {
                let entry = entry?;
                println!("\t> {}", entry.file_name().to_string_lossy());
            }
            print!("\n");
            Ok(())
        },
        "++" => {
            File::create(format!("new_file.{}", args.next().unwrap()))?;
            Ok(())
        },
       "--" => {
            let file_name = match args.next() {
                Some(name) => name,
                None => return Err(Error::new(ErrorKind::InvalidInput, "No file specified")),
            };

            println!("Are you sure you want to delete {} \n-> (yes/no)?", file_name);

            let dir = env::current_dir()?;
            let full_path = dir.join(file_name);

            if !full_path.exists() {
                return Err(Error::new(ErrorKind::NotFound, "File not found"));
            }

            let mut input = String::new();
            io::stdin().read_line(&mut input)?;

            if input.trim() == "yes" {
                std::fs::remove_file(&full_path)?;
                println!("File deleted: {}", file_name);
            } else {
                println!("Deletion canceled.");
            }

            Ok(())
        },
        "exit" => std::process::exit(0),
        _ => Err(Error::new(ErrorKind::NotFound, "Command not found")),
    }
}

