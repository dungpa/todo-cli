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
    List,
    Reset,
}

pub fn parse_command() -> Command {
    return Command::parse();
}