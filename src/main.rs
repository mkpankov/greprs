extern crate greprs;

use std::env;
use std::io::{self, Write};

fn main() {
    let args: Vec<_> = env::args().collect();
    let maybe_cfg = greprs::parse_config(&args);
    let cfg = match maybe_cfg {
        Err(greprs::ParseConfigError::NotEnoughArgs) => {
            writeln!(io::stderr(), "USAGE: greprs <pattern> <file-name>").unwrap();
            std::process::exit(1)
        }
        Ok(cfg) => cfg,
    };

    println!(
        "Searching a needle '{}' in a haystack '{}'",
        cfg.needle,
        cfg.haystack
    );

    greprs::search(&cfg.haystack, &cfg.needle);
}
