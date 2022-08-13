mod domain;
mod backend;

fn main() {
    let args = domain::parse_command();
    match backend::execute(args) {
        Ok(msgs) => { 
            for msg in msgs { 
                println!("{}", msg); 
            }

            std::process::exit(0); 
        },
        Err(err) => { 
            eprintln!("{}", err); 
            std::process::exit(1); 
        }
    };
}
