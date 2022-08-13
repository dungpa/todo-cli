use clap::Parser;
use std::fs::OpenOptions;
use std::io::prelude::*;

#[derive(Parser)]
#[derive(Debug)]
struct AddCommand {
    todo: String
}

#[derive(Parser)]
#[derive(Debug)]
enum Command {
    Add(AddCommand),
}

fn main() {
    let args = Command::parse();
    println!("Arguments: {:?}", args);
    match args {
        Command::Add(AddCommand { todo }) => {
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
