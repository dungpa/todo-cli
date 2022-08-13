use std::fs::OpenOptions;
use std::io::prelude::*;
mod domain;

fn main() {
    let args = domain::parse_command();
    println!("Arguments: {:?}", args);
    match args {
        domain::Command::Add(domain::AddCommand { todo }) => {
            let mut file = OpenOptions::new()
                            .create(true)
                            .append(true)
                            .open("todo.txt")
                            .unwrap();

            if let Err(e) = writeln!(file, "{}", todo) {
                eprintln!("Couldn't write to file: {}", e);
            } else {
                println!("TODO item '{}' added.", todo)
            }
        }
    }
}
