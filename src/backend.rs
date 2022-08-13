use std::fs;
use std::fs::OpenOptions;
use std::io::prelude::*;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use anyhow::{anyhow, Context, Result};

use crate::domain::*;

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

pub fn execute(cmd: Command) -> Result<Vec<String>, anyhow::Error> {
    let data_store = "todo.txt";
    let mut results = vec![];
    match cmd {
        Command::Add(AddCommand { todo }) => {
            let mut file = OpenOptions::new()
                            .create(true)
                            .append(true)
                            .open(data_store)
                            .unwrap();

            writeln!(file, "[ ] {}", todo)
                .with_context(|| format!("Couldn't write to file: {}", data_store))?;
            results.push(format!("TODO item '{}' added.", todo));
            Ok(results)
        },
        Command::Remove(RemoveCommand { index }) => {
            let todos = read_lines(data_store)?;
            
            let mut remaining = vec![];
            let mut index_found = false;
            for (i, todo) in todos.flatten().enumerate() {
                if i + 1 != index.try_into().unwrap() {
                    remaining.push(todo);
                } else {
                    index_found = true;
                }
            }
            if !index_found {
                Err(anyhow!("Unable to find TODO item {}.", index))
            } else {
                fs::write(data_store, remaining.join("\n")).expect("Unable to write to file.");

                results.push(format!("TODO item no. {} removed.", index));
                Ok(results)
            }
        },
        Command::List => {
            let todos = read_lines(data_store)?;
            for (i, todo) in todos.flatten().enumerate() {
                results.push(format!("{}. {}", i + 1, todo));
            }
            results.push("TODO items listed.".into());
            Ok(results)
        },
        Command::Reset => {
            fs::remove_file(data_store)?;
            results.push("All TODO items deleted.".into());
            Ok(results)
        },
    }
}