use std::io::{stdin, stdout, Write};
use std::process::Command;

fn main() {

    loop {
        print!("[<Command>] ");
        stdout().flush().unwrap();
        
        let mut input = String::new();
    
        stdin().read_line(&mut input).unwrap();
    
        let mut parts = input.trim().split_whitespace();
        let command = parts.next().unwrap();
        let args = parts;

    
        let mut child = Command::new(command).args(args).spawn().unwrap();
        
        child.wait().unwrap();
    }


}
