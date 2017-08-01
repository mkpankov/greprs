use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::io::{self, Write};
use std::iter::Iterator;
use std::error::Error;

pub struct Config {
    pub needle: String,
    pub haystack: String,
}

pub enum ParseConfigError {
    NotEnoughArgs,
}

pub fn parse_config(args: &[String]) -> Result<Config, ParseConfigError> {
    if args.len() == 3 {
        let needle = args[1].clone();
        let haystack = args[2].clone();
        Ok(Config {
            needle: needle,
            haystack: haystack,
        })
    } else {
        Err(ParseConfigError::NotEnoughArgs)
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Match {
    line: usize,
    span: (usize, usize),
}

fn search_impl<T>(lines: T, needle: &str) -> Vec<Match>
where
    T: Iterator<Item = String>,
{
    let mut matches = Vec::new();
    for (line_number, line) in lines.enumerate() {
        let index = line.find(needle);
        match index {
            Some(offset) => {
                matches.push(Match {
                    line: line_number + 1,
                    span: (offset, offset + needle.len()),
                })
            }
            None => {
                continue;
            }
        }
    }
    matches
}

pub fn search(haystack: &str, needle: &str) {
    let maybe_file = File::open(haystack);
    let file;
    match maybe_file {
        Err(e) => {
            writeln!(io::stderr(), "Error: {}.", e.description()).unwrap();
            // XXX: This exit breaks tests
            std::process::exit(1);
        }
        Ok(f) => {
            file = f;
        }
    }
    let reader = BufReader::new(file);
    let lines = reader.lines().take_while(|x| x.is_ok()).map(|x| x.unwrap());
    for i in search_impl(lines, needle).iter() {
        println!("{} found @ line {}", needle, i.line);
    }
}

#[test]
fn search_directory_nonrecursively() {
    let cur_dir = std::env::current_dir().unwrap();
    // XXX: This test is bad, it only makes sure we don't loop indefinitely
    search(cur_dir.to_str().unwrap(), "foo");
}

#[test]
fn search_one_entry() {
    // one entry in input
    let lines = vec![
        String::from("bla"),
        String::from("zxc"),
        String::from("qwe"),
    ];
    let matches = search_impl(lines.into_iter(), "zxc");
    assert_eq!(
        matches[0],
        Match {
            line: 2,
            span: (0, 3),
        }
    );
}

#[test]
fn search_empty_input() {
    // empty input
    let lines = Vec::new();
    let matches = search_impl(lines.into_iter(), "zxc");
    assert!(matches.is_empty());
}

#[test]
fn search_two_entries() {
    // two entries in input
    let lines = vec![
        String::from("bla"),
        String::from("zxc"),
        String::from("qwe"),
        String::from("bla"),
    ];
    let matches = search_impl(lines.into_iter(), "bla");
    assert_eq!(matches.len(), 2);
    assert_eq!(
        matches[0],
        Match {
            line: 1,
            span: (0, 3),
        }
    );
    assert_eq!(
        matches[1],
        Match {
            line: 4,
            span: (0, 3),
        }
    );
}

#[test]
fn search_no_entries() {
    // no entries in non-empty input
    let lines = vec![String::from("zxc"), String::from("asd")];
    let matches = search_impl(lines.into_iter(), "bla");
    assert!(matches.is_empty());
}

#[test]
fn search_cyrilic_entry() {
    // one cyrilic entry
    let lines = vec![
        String::from("йцу"),
        String::from("фыв"),
        String::from("ячс"),
    ];
    let matches = search_impl(lines.into_iter(), "фыв");
    assert_eq!(
        matches[0],
        Match {
            line: 2,
            // Issue #3
            span: (0, 6),
        }
    );
}
