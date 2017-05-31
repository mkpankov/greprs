extern crate greprs;

use std::env;
use std::fs::File;
use std::io::prelude::*;

fn main() {
    let args: Vec<_> = env::args().collect();
    let cfg = greprs::parse_config(&args);
    println!("Searching a needle '{}' in a haystack '{}'", cfg.needle, cfg.haystack);
/*    
    let mut file = File::open(haystack).expect("File not found");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Can't read a file");
    //assert_eq!(contents, "Hello, world!");
    print!("{}", contents);
*/
}
