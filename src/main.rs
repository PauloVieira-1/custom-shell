mod commandHandler;

use commandHandler::handleCommand;
use std::io::{stdin, stdout, Write, Result};
use std::process::Command;

fn main() -> Result<()> {
    loop {
        print!("[<Command>] ");
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
