pub struct Config {
    pub needle: String,
    pub haystack: String,
}

pub fn parse_config(args: &[String]) -> Config {
    let needle = args[1].clone();
    let haystack = args[2].clone();
    Config { needle: needle, haystack: haystack }
}
