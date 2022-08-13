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

const DATA_STORE: &str = "todo.txt";
const PENDING_PREFIX: &str = "[ ]";
const COMPLETED_PREFIX: &str = "[x]";

#[cfg(windows)]
const LINE_ENDING: &'static str = "\r\n";
#[cfg(not(windows))]
const LINE_ENDING: &'static str = "\n";

fn add(todo: String) -> Result<Vec<String>, anyhow::Error> {
    let mut results = vec![];
    let mut file = OpenOptions::new()
                            .create(true)
                            .append(true)
                            .open(DATA_STORE)
                            .unwrap();

    writeln!(file, "{} {}", PENDING_PREFIX, todo)
        .with_context(|| format!("Couldn't append to file: {}", DATA_STORE))?;
    results.push(format!("TODO item '{}' added.", todo));
    Ok(results)
}

/// Index is assumed to start from 1.
fn remove(index: u32) -> Result<Vec<String>, anyhow::Error> {
    let mut results = vec![];
    let todos = read_lines(DATA_STORE)?;
            
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
        // This is inefficient since we read all entries, remove one entry and save all others back to file.
        fs::write(DATA_STORE, remaining.join(LINE_ENDING))
            .with_context(|| format!("Couldn't write to file: {}", DATA_STORE))?;

        results.push(format!("TODO item #{} removed.", index));
        Ok(results)
    }
}

fn complete(index: u32) -> Result<Vec<String>, anyhow::Error> {
    let mut results = vec![];
    let todos = read_lines(DATA_STORE)?;
            
    let mut transformed = vec![];
    let mut index_found = false;
    for (i, todo) in todos.flatten().enumerate() {
        if i + 1 != index.try_into().unwrap() {
            transformed.push(todo);
        } else {
            index_found = true;
            transformed.push(todo.replace(PENDING_PREFIX, COMPLETED_PREFIX));
        }
    }
    if !index_found {
        Err(anyhow!("Unable to find TODO item {}.", index))
    } else {
        // This is inefficient since we read all entries, transform one entry and save all of them back to file.
        fs::write(DATA_STORE, transformed.join(LINE_ENDING))
            .with_context(|| format!("Couldn't write to file: {}", DATA_STORE))?;

        results.push(format!("TODO item #{} completed.", index));
        Ok(results)
    }
}

fn display(todo_type: TodoType) -> Result<Vec<String>, anyhow::Error> {
    let mut results = vec![];
    let todos = read_lines(DATA_STORE)?;
    for (i, todo) in todos.flatten().enumerate() {
        if todo_type == TodoType::All || todo.starts_with(PENDING_PREFIX) {
            results.push(format!("{}. {}", i + 1, todo));
        }
    }
    results.push(format!("{:?} TODO items displayed.", todo_type));
    Ok(results)
}

fn stats() -> Result<Vec<String>, anyhow::Error> {
    let mut results = vec![];
    let mut pending_count = 0;
    let mut completed_count = 0;
    let todos = read_lines(DATA_STORE)?;
    for todo in todos.flatten() {
        if todo.starts_with(PENDING_PREFIX) {
            pending_count += 1;
        } else if todo.starts_with(COMPLETED_PREFIX) {
            completed_count += 1;
        }
    }
    results.push(format!("Pending: {} TODO item(s), completed: {} TODO item(s).", pending_count, completed_count));
    Ok(results)
}

fn reset() -> Result<Vec<String>, anyhow::Error> {
    let mut results = vec![];
    fs::remove_file(DATA_STORE)?;
    results.push("All TODO items deleted.".into());
    Ok(results)
}

pub fn execute(cmd: Command) -> Result<Vec<String>, anyhow::Error> {
    match cmd {
        Command::Add(ContentCommand { todo }) => {
            add(todo)
        },
        Command::Remove(IndexCommand { index }) => {
            remove(index)
        },
        Command::Complete(IndexCommand { index }) => {
            complete(index)
        },
        Command::List => {
            display(TodoType::Pending)
        },
        Command::Audit => {
            display(TodoType::All)
        },
        Command::Stats => {
            stats()
        },
        Command::Reset => {
            reset()
        },
    }
}