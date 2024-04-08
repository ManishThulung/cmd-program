use std::env;
use std::process;
use grep::Config;

fn main() {
    let args: Vec<String> = env::args().collect();
    
    let config = Config::build(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing args: {err}");
        process::exit(1);
    });


    println!("searching {} in {} file", config.query, config.file_path);

    if let Err(e) = grep::run(config) {
        eprintln!("Application error: {e}");
        process::exit(1);
    };
}


