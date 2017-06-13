use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::iter::Iterator;

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
}

fn search_impl<T>(lines: T, needle: &str) -> Vec<Match>
    where T: Iterator<Item = String>
{
    let mut matches = Vec::new();
    for (i, line) in lines.enumerate() {
        if line.contains(&needle) {
            matches.push(Match { line: i + 1 });
        }
    }
    matches
}

pub fn search(haystack: &str, needle: &str) {
    let file = File::open(haystack).expect("File not found");
    let reader = BufReader::new(file);
    let lines = reader.lines().filter_map(|s| s.ok());
    for i in search_impl(lines, needle).iter() {
        println!("{} found @ line {}", needle, i.line);
    }
}

#[test]
fn search_one_entry() {
    // one entry in input
    let lines = vec![String::from("bla"), String::from("zxc"), String::from("qwe")];
    let matches = search_impl(lines.into_iter(), "zxc");
    assert_eq!(matches[0], Match { line: 2 });
}

#[test]
fn search_empty_input() {
    // empty input
    let lines = Vec::new();
    let matches = search_impl(lines.into_iter(), "zxc");
    assert_eq!(matches.is_empty(), true);
}

#[test]
fn search_two_entries() {
    // two entries in input
    let lines =
        vec![String::from("bla"), String::from("zxc"), String::from("qwe"), String::from("bla")];
    let matches = search_impl(lines.into_iter(), "bla");
    assert_eq!(matches.len(), 2);
    assert_eq!(matches[0], Match { line: 1 });
    assert_eq!(matches[1], Match { line: 4 });
}

#[test]
fn search_no_entries() {
    // no entries in non-empty input
    let lines = vec![String::from("zxc"), String::from("asd")];
    let matches = search_impl(lines.into_iter(), "bla");
    assert_eq!(matches.is_empty(), true);
}

#[test]
fn search_cyrilic_entry() {
    // one cyrilic entry
    let lines = vec![String::from("йцу"), String::from("фыв"), String::from("ячс")];
    let matches = search_impl(lines.into_iter(), "фыв");
    assert_eq!(matches[0], Match { line: 2 });
}
