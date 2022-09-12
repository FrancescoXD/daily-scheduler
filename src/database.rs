//use rusqlite::{params, Connection, Result};
use rusqlite::{Connection, Result};

pub struct Database {
    path: String,
}

impl Database {
    pub fn new(file: &str) -> Database {
        Database {
            path: file.to_string(),
        }
    }

    pub fn check_path(&self) -> bool {
        // change exists with try_exists when it exits from nightly build
        std::path::Path::new(&self.path).exists()
    }

    pub fn create_default_database(&self) -> Result<()> {
        let db = Connection::open(self.path.clone()).expect("unable to open database");
        db.execute("create table if not EXISTS scheduler (day TEXT, hour_start TEX, hour_end TEXT, description TEXT);", ()).unwrap();

        Ok(())
    }
}
