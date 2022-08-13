use clap::Parser;

#[derive(Parser)]
#[derive(Debug)]
pub struct ContentCommand {
    pub todo: String
}

#[derive(Parser)]
#[derive(Debug)]
pub struct IndexCommand {
    pub index: u32
}

#[derive(Parser)]
#[derive(Debug)]
pub enum Command {
    Add(ContentCommand),
    Remove(IndexCommand),
    /// Show pending TODOs
    List,
    /// Show all TODOs including pending and completed ones
    Audit,
    Reset,
}

#[derive(Debug)]
#[derive(PartialEq)]
pub enum TodoType {
    Pending,
    All
}

pub fn parse_command() -> Command {
    return Command::parse();
}