use std::error::Error;
use std::fs;

pub struct Config {
    pub opt: String,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        let mut opt: String = "".to_string();
        if args.len() > 1 {
            opt = args[1].clone();
        }
        Ok(Config { opt })
    }
}

pub fn run(_config: Config) -> Result<(), Box<dyn Error>> {
    let files = fs::read_dir("./")
        .unwrap()
        .take_while(|file| file.is_ok())
        .map(|file| file.unwrap());
    for file in files {
        println!("{}", file.path().display());
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_config() {
        let args: [String; 2] = ["PATH_TO_THE_FILE".to_string(), "x".to_string()];
        let config = Config::new(&args).unwrap();
        assert_eq!("x".to_string(), config.opt);
    }
}
