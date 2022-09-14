use configparser::ini::Ini;
use std::fs;
use std::io::prelude::*;

pub struct Config {
    config: Ini,
    file: String,
}

impl Config {
    pub fn new(file: &str) -> Config {
        Config {
            config: Ini::new(),
            file: file.to_string(),
        }
    }

    pub fn create_default_config(&self) -> Result<(), &str> {
        let mut file = fs::File::create(self.file.clone()).expect("could not create file");
        if let Err(_) = file.write_all(b"[colors]\ndescription = BLUE\nhours = GREEN\n\n[database]\npath = ./data.db\n") {
            Err("unable to make default config file")
        } else {
            Ok(())
        }
    }

    pub fn check_path(&self) -> bool {
        // change exists with try_exists when it exits from nightly build
        std::path::Path::new(&self.file).exists()
    }

    pub fn load_config(&mut self) -> Result<(), &str> {
        match self.config.load(self.file.clone()) {
            Ok(_) => Ok(()),
            Err(_) => Err("unable to load config file"),
        }
    }

    pub fn get(&self, desc: &str, key: &str) -> Option<String> {
        self.config.get(desc, key)
    }
}
