use std::fs::OpenOptions;
use std::io::prelude::*;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use anyhow::{Context, Result};

use crate::domain::*;

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

pub fn execute(cmd: Command) -> Result<String, anyhow::Error> {
    let data_store = "todo.txt";
    match cmd {
        Command::Add(AddCommand { todo }) => {
            let mut file = OpenOptions::new()
                            .create(true)
                            .append(true)
                            .open(data_store)
                            .unwrap();

            writeln!(file, "[ ] {}", todo)
                .with_context(|| format!("Couldn't write to file: {}", data_store))?;
            Ok(format!("TODO item '{}' added.", todo))
        },
        Command::List => {
            let todos = read_lines("todo.txt")?;
            for todo in todos.flatten() {
                println!("{}", todo);
            }
            Ok("TODO items listed.".into())
        }
    }
}