mod domain;
mod backend;

fn main() {
    let args = domain::parse_command();
    match backend::execute(args) {
        Ok(msgs) => { for msg in msgs { println!("{}", msg); } },
        Err(err) => { eprintln!("{}", err); }
    };
}
