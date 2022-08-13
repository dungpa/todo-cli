use std::fs::OpenOptions;
use std::io::prelude::*;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

use crate::domain::*;

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

pub fn execute(cmd: Command) {
    match cmd {
        Command::Add(AddCommand { todo }) => {
            let mut file = OpenOptions::new()
                            .create(true)
                            .append(true)
                            .open("todo.txt")
                            .unwrap();

            if let Err(e) = writeln!(file, "[ ] {}", todo) {
                eprintln!("Couldn't write to file: {}", e);
            } else {
                println!("TODO item '{}' added.", todo)
            }
        },
        Command::List => {
            if let Ok(todos) = read_lines("todo.txt") {
                for todo in todos.flatten() {
                    println!("{}", todo);
                }
            } else {
                eprintln!("Couldn't read file");
            }
        }
    }
}