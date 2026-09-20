use std::env;

pub struct Config {
    pub files_dir: String,
}

impl Config {
    pub fn parse() -> Self {
        let env_args: Vec<String> = env::args().collect();
        let files_dir = if env_args.len() > 2 {
            env_args[2].clone()
        } else {
            ".".to_string()
        };

        Config { files_dir }
    }
}