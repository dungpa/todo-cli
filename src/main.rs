use clap::Parser;

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

#[derive(Parser)]
#[derive(Debug)]
struct Arguments {
    /// Command to execute
    command: String,
    /// Content of the TODO
    todo_content: String,
}

fn main() {
    let args = Command::parse();
    println!("Arguments, {:?}", args);
}
