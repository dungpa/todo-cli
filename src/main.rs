mod domain;
mod backend;

fn main() {
    let args = domain::parse_command();
    match backend::execute(args) {
        Ok(msg) => { println!("{}", msg); },
        Err(err) => { eprintln!("{}", err); }
    };
}
