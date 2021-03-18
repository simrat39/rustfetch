use std::process;

use rustfetch::run;

fn main() {
    run().unwrap_or_else(|err| {
        eprintln!("An error occured: {}", err);
        process::exit(1);
    });
}
