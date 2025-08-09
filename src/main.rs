mod commandHandler;

use commandHandler::handleCommand;
use std::io::{stdin, stdout, Write, Result};
use std::process::Command;

fn main() -> Result<()> {

    loop {
        print!("[<Command>] ");
        stdout().flush().unwrap();
        
        let mut input = String::new();
    
        stdin().read_line(&mut input).unwrap();
    
        let mut parts = input.trim().split_whitespace();
        let command = parts.next().unwrap();
        let args = parts;

        handleCommand(command.to_string(), &args);

        let mut child = Command::new(command).args(args).spawn().unwrap();
        child.wait().unwrap();
    }

    Ok(())
}
