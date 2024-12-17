use minigrep::Config;
#[warn(dead_code)]
use std::{env, process};
fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::build(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    match minigrep::run(config) {
        Ok(_) => {}
        Err(e) => {
            eprintln!("Application error : {e}")
        }
    }
}