use clap::Parser;

#[derive(Parser)]
#[derive(Debug)]
pub struct AddCommand {
    pub todo: String
}

#[derive(Parser)]
#[derive(Debug)]
pub enum Command {
    Add(AddCommand),
}

pub fn parse_command() -> Command {
    return Command::parse();
}