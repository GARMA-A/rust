use minigrep::Config;
use std::{env, process};
fn main() {
    let args = env::args().collect::<Vec<String>>().into_iter();
    let config = Config::build(args).unwrap_or_else(|err| {
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
