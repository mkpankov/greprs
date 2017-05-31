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
