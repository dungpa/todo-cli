use clap::Parser;

#[derive(Parser)]
#[derive(Debug)]
pub struct AddCommand {
    pub todo: String
}

#[derive(Parser)]
#[derive(Debug)]
pub struct RemoveCommand {
    pub index: u32
}

#[derive(Parser)]
#[derive(Debug)]
pub enum Command {
    Add(AddCommand),
    Remove(RemoveCommand),
    /// Show pending TODOs
    List,
    /// Show all TODOs including pending and completed ones
    Audit,
    Reset,
}

pub fn parse_command() -> Command {
    return Command::parse();
}