use clap::Parser;

#[derive(Parser, Debug)]
pub struct ContentCommand {
    pub todo: String,
}

#[derive(Parser, Debug)]
pub struct IndexCommand {
    pub index: u32,
}

#[derive(Parser, Debug)]
pub enum Command {
    Add(ContentCommand),
    Remove(IndexCommand),
    Complete(IndexCommand),
    /// Show pending TODOs
    List,
    /// Show all TODOs including pending and completed ones
    Audit,
    Stats,
    Reset,
}

#[derive(Debug, PartialEq)]
pub enum TodoType {
    Pending,
    All,
}

pub fn parse_command() -> Command {
    Command::parse()
}
