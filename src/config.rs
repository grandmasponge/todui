use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Config {
    version: u8,
    path: String,
}

impl ::std::default::Default for Config {
    fn default() -> Self { Self { version: 1, path: "recources/todo.json".parse().unwrap() } }
}


impl Config {
    pub fn new() -> Self {
        let config: Config = confy::load("todui", None).unwrap();
        config
    }
}