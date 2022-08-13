mod domain;
mod backend;

fn main() {
    let args = domain::parse_command();
    backend::execute(args);
}
