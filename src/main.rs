extern crate greprs;

use std::env;
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

fn main() {
    let args: Vec<_> = env::args().collect();
    let maybe_cfg = greprs::parse_config(&args);
    let cfg = match maybe_cfg {
        Err(greprs::ParseConfigError::NotEnoughArgs) => {
            println!("USAGE: greprs <pattern> <file-name>");
            std::process::exit(1)
        }
        Ok(cfg) => {
            cfg
        }
    };

    println!("Searching a needle '{}' in a haystack '{}'", cfg.needle, cfg.haystack);

    let file = File::open(cfg.haystack).expect("File not found");
    let reader = BufReader::new(file);
    for (i, line) in reader.lines().enumerate() {
        let l = line.unwrap();
        if l.contains(&cfg.needle) {
            println!("{} found @ line {}", l, i + 1);
        }
    }

}
