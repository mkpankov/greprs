extern crate greprs;

use std::env;
use std::io::{self, Write};

fn usage() {
    writeln!(
        io::stderr(),
        "USAGE: greprs [OPTIONS] <pattern> <file-name>"
    ).unwrap();
    writeln!(io::stderr(), "OPTIONS:").unwrap();
    writeln!(io::stderr(), "\t-r\trecursive").unwrap();
}

fn main() {
    let args: Vec<_> = env::args().collect();
    let maybe_cfg = greprs::parse_config(&args);
    let cfg = match maybe_cfg {
        Err(greprs::ParseConfigError::UnknownOpt) => {
            // TODO: print value of unknown option
            writeln!(io::stderr(), "Unknown option").unwrap();
            usage();
            std::process::exit(1);
        }
        Err(greprs::ParseConfigError::NotEnoughArgs) => {
            usage();
            std::process::exit(1);
        }
        Ok(cfg) => cfg,
    };

    if !cfg.recursive {
        println!(
            "Searching a needle '{}' in a haystack '{}'",
            cfg.needle,
            cfg.haystack
        );
        greprs::search(&cfg.haystack, &cfg.needle);
    } else {
        println!(
            "Searching recursive a needle '{}' in a haystack '{}'",
            cfg.needle,
            cfg.haystack
        );
        greprs::search_recursive(&cfg.haystack, &cfg.needle);
    }
}
