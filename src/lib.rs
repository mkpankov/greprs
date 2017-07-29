use std::fs::File;
use std::fs;
use std::path::{Path, PathBuf};
use std::io::BufReader;
use std::io::prelude::*;
use std::io::{self, Write};
use std::iter::Iterator;
use std::error::Error;

pub struct Config {
    pub needle: String,
    pub haystack: String,
    pub recursive: bool,
}

pub enum ParseConfigError {
    NotEnoughArgs,
    UnknownOpt,
}

pub fn parse_config(args: &[String]) -> Result<Config, ParseConfigError> {
    let mut opts = 0;
    let mut recursive = false;
    if args.len() == 4 {
        if args[1] == "-r" {
            recursive = true;
        } else {
            return Err(ParseConfigError::UnknownOpt);
        }
        opts = 1;
    }
    if args.len() - opts == 3 {
        let needle = args[opts + 1].clone();
        let haystack = args[opts + 2].clone();
        Ok(Config {
            needle: needle,
            haystack: haystack,
            recursive: recursive,
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

struct WalkDir {
    dirs: Vec<PathBuf>,
    walk_dir_it: std::fs::ReadDir,
}

impl WalkDir {
    fn new<P: AsRef<Path>>(path: P) -> io::Result<WalkDir> {
        let dir = fs::read_dir(path)?;
        Ok(WalkDir {
            dirs: Vec::new(),
            walk_dir_it: dir,
        })
    }
}

impl Iterator for WalkDir {
    type Item = io::Result<PathBuf>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.walk_dir_it.next() {
            Some(maybe_entry) => {
                match maybe_entry {
                    Err(entry) => Some(Err(entry)),
                    Ok(entry) => {
                        let path = entry.path();
                        if path.is_dir() {
                            self.dirs.push(path);
                            self.next()
                        } else {
                            Some(Ok(path))
                        }
                    }
                }
            }
            None => {
                match self.dirs.pop() {
                    Some(e) => {
                        match fs::read_dir(&e.as_path()) {
                            Err(p) => Some(Err(p)),
                            Ok(p) => {
                                self.walk_dir_it = p;
                                self.next()
                            }
                        }
                    }
                    None => None,
                }
            }
        }
    }
}

pub fn search_recursive(haystack: &str, needle: &str) {
    if Path::new(haystack).is_file() {
        search(haystack, needle);
    } else {
        match WalkDir::new(&haystack) {
            Err(e) => {
                writeln!(io::stderr(), "Error: {}.", e.description()).unwrap();
                std::process::exit(1);
            }
            Ok(walkdir) => {
                for entry in walkdir {
                    match entry {
                        Err(e) => {
                            writeln!(io::stderr(), "Error: {}.", e.description()).unwrap();
                        }
                        Ok(e) => {
                            writeln!(io::stderr(), "File: {}", e.to_str().unwrap()).unwrap();
                            search(e.to_str().unwrap(), needle);
                        }
                    }
                }
            }
        }
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
            span: (0, 3),
        }
    );
}

#[test]
fn recursive_walk() {
    // TODO: find better way to choose test data path
    let test_data_path = std::env::current_dir().unwrap();
    let files: Vec<_> = WalkDir::new(&test_data_path).unwrap().collect();
    let matches: Vec<_> = files
        .into_iter()
        .filter(|x| {
            x.as_ref()
                .unwrap()
                .as_path()
                .to_str()
                .unwrap()
                .find("haystack.txt")
                .is_some()
        })
        .collect();
    assert_eq!(matches.len(), 3);
}
