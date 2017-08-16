extern crate greprs;
extern crate clap;

use clap::{Arg, App};

fn main() {
    let matches = App::new("greprs")
        .about("grep implementation")
        .version("0.1.0")
        .arg(
            Arg::with_name("pattern")
                .help("pattern for search")
                .required(true)
                .index(1),
        )
        .arg(
            Arg::with_name("file-name")
                .help("file or directiry for search")
                .required(true)
                .index(2),
        )
        .arg(Arg::with_name("r").short("r").help("recursive search"))
        .get_matches();

    let needle = matches.value_of("pattern").unwrap();
    let haystack = matches.value_of("file-name").unwrap();

    if !matches.is_present("r") {
        println!(
            "Searching a needle '{}' in a haystack '{}'",
            needle,
            haystack
        );
        greprs::search(&haystack, &needle);
    } else {
        println!(
            "Searching recursive a needle '{}' in a haystack '{}'",
            needle,
            haystack
        );
        greprs::search_recursive(&haystack, &needle);
    }
}
