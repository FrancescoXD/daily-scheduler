//use rusqlite::{params, Connection, Result};
use chrono::prelude::*;
use rusqlite::{Connection, Result};

pub struct Database {
    path: String,
    today: DateTime<Local>,
}

impl Database {
    pub fn new(file: &str) -> Database {
        Database {
            path: file.to_string(),
            today: Local::now(),
        }
    }

    pub fn check_path(&self) -> bool {
        // change exists with try_exists when it exits from nightly build
        std::path::Path::new(&self.path).exists()
    }

    pub fn create_default_database(&self) -> Result<(), &str> {
        let db = Connection::open(self.path.clone()).expect("unable to open database");
        match db.execute("create table if not EXISTS scheduler (day TEXT, hour_start TEXT, hour_end TEXT, description TEXT);", ()) {
            Ok(_) => Ok(()),
            Err(_) => Err("unable to create database table"),
        }
    }

    pub fn get_local_datetime(&self) -> String {
        String::from(&self.today.format("%Y-%m-%d").to_string())
    }

    pub fn insert_test(&self) {
        let db = Connection::open(self.path.clone()).unwrap();
        db.execute("insert into scheduler (day, hour_start, hour_end, description) values (?1, ?2, ?3, ?4)", (self.get_local_datetime(), "10:30", "11:30", "worked on scheduler")).unwrap();
    }
}
