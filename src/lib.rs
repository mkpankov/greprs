use std::fs::File;
use std::fs;
use std::path::{Path, PathBuf};
use std::io::BufReader;
use std::io::prelude::*;
use std::io::{self, Write};
use std::iter::Iterator;
use std::error::Error;

#[derive(Debug, PartialEq, Eq)]
struct Match {
    line: usize,
    line_match: LineMatch,
}

#[derive(Debug, PartialEq, Eq)]
struct LineMatch {
    span_bytes: (usize, usize),
    span_chars: (usize, usize),
}

fn determine_needle_char_bounds(
    haystack: &str,
    needle_byte_bounds: (usize, usize),
) -> (usize, usize) {
    let (needle_start, needle_end) = needle_byte_bounds;
    let mut needle_start_char_index = None;
    let mut needle_end_char_index = None;
    let mut char_index = 0;
    for (byte_index, _) in haystack.char_indices() {
        if byte_index == needle_start {
            needle_start_char_index = Some(char_index);
        }
        if byte_index == needle_end {
            needle_end_char_index = Some(char_index);
        }
        char_index += 1;
    }

    if needle_end_char_index.is_none() {
        needle_end_char_index = Some(char_index);
    }

    (
        needle_start_char_index.unwrap(),
        needle_end_char_index.unwrap(),
    )
}

fn search_line_impl(haystack: &str, needle: &str) -> Option<LineMatch> {
    let maybe_needle_start = haystack.find(needle);

    let needle_start = match maybe_needle_start {
        Some(o) => o,
        None => return None,
    };
    let needle_end = needle_start + needle.len();

    let needle_char_bounds = determine_needle_char_bounds(haystack, (needle_start, needle_end));

    Some(LineMatch {
        span_bytes: (needle_start, needle_end),
        span_chars: needle_char_bounds,
    })
}

fn search_impl<T>(lines: T, needle: &str) -> Vec<Match>
where
    T: Iterator<Item = String>,
{
    let mut matches = Vec::new();
    for (line_number, line) in lines.enumerate() {
        let maybe_line_match = search_line_impl(&line, needle);
        match maybe_line_match {
            Some(line_match) => {
                matches.push(Match {
                    line: line_number + 1,
                    line_match: line_match,
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
            line_match: LineMatch {
                span_bytes: (0, 3),
                span_chars: (0, 3),
            },
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
            line_match: LineMatch {
                span_bytes: (0, 3),
                span_chars: (0, 3),
            },
        }
    );
    assert_eq!(
        matches[1],
        Match {
            line: 4,
            line_match: LineMatch {
                span_bytes: (0, 3),
                span_chars: (0, 3),
            },
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
            line_match: LineMatch {
                span_bytes: (0, 6),
                span_chars: (0, 3),
            },
        }
    );
}

#[test]
fn recursive_walk() {
    let program_path = std::env::current_exe().unwrap();
    let test_data_path = program_path
        .parent()
        .unwrap()
        .parent()
        .unwrap()
        .parent()
        .unwrap()
        .parent()
        .unwrap();
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
