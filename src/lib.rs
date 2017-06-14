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
    if args.len() < 2 {
        return Err(ParseConfigError::NotEnoughArgs);
    }
    let needle = args[1].clone();
    let haystack = args[2].clone();
    Ok( Config { needle: needle, haystack: haystack } )
}

#[derive(Debug, PartialEq, Eq)]
struct Match {
    line: usize,
}

fn search_impl<T>(lines: T, needle: &str) -> Vec<Match>
    where T: Iterator<Item = String>
{
    lines
    .enumerate()
    .filter_map(|(i, line)|
        if line.contains(needle) {Some(Match{line: i + 1})} else {None}
    )
    .collect()
}

pub fn search(haystack: &str, needle: &str) {
    let file = File::open(haystack).expect("File not found");
    let reader = BufReader::new(file);
    let lines = reader.lines().filter_map(Result::ok);
    search_impl(lines, needle);
}

#[test]
fn search_works() {
    let lines = vec![
        String::from("foo"),
        String::from("bar"),
        String::from("baz")
    ];
    let matches = search_impl(lines.into_iter(), "bar");
    assert_eq!(matches[0], Match { line: 2 });
}

#[test]
fn search_empty() {
    assert_eq!(search_impl(vec![].into_iter(), "bar"), []);
}

#[test]
fn search_return_many_cyr() {
    let lines = vec![
        String::from("фу"),
        String::from("бар"),
        String::from("баз")
    ];
    let matches = search_impl(lines.into_iter(), "ба");
    assert_eq!(matches, [Match{line: 2}, Match{line: 3}]);
}

#[test]
fn search_none_from_some() {
    let lines = vec![
        String::from("foo"),
        String::from("bar"),
        String::from("baz")
    ];
    let matches = search_impl(lines.into_iter(), "qux");
    assert_eq!(matches, []);
}
