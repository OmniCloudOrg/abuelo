use std::{rc::Rc, sync::Arc};

use chrono::{DateTime, Utc};
use rusqlite::{Connection, Result};
use sha2::{Digest, Sha256};

pub struct Database {
    conn: Connection,
}

impl Database {
    pub fn new() -> Self {
        let conn = Connection::open("user_db.db3").unwrap();

        // If this returns an error; it's prolly cuz the table already exists.
        // That's fine and we can just let it error and the other queries will 
        // use the existing table instead
        // NOTE: Look at the other possible errors here
        let _ = conn.execute(
            "CREATE TABLE users (
            user_id           INTEGER PRIMARY KEY,
            username          TINYTEXT NOT NULL,
            password_hash     TINYTEXT NOT NULL,
            creation_time     DATETIME NOT NULL
            is_premium        BOOL NOT NULL,
            random_value      INTEGER NOT NULL
        )",
            ());
        Self { conn }
    }

    pub fn add_user(&self, username : &str, password : &str) -> Result<()>{
        let password_hash = self.hash_password(username, password)?;
        self.conn.execute(
            "INSERT INTO person (
            username, 
            password_hash, 
            creation_time, 
            is_premium,
            random_value) 
            VALUES (?1, ?2, ?3, ?4, ?5)",
            (username, password_hash, Utc::now(), false, rand::random::<u64>()),
        )?;
        Ok(())
    }

    fn hash_password(&self, username : &str, password : &str) -> Result<String>{
        let (creation_time, num) : (DateTime<Utc>, u64)  = self.conn.query_row(
            "SELECT creation_time, random_value, name FROM person WHERE name=?1",
            [username],
            |row|{
                let creation_time = row.get(0)?;
                let num = row.get(1)?;
                Ok((creation_time, num))
            },
        )?;
        let mut hasher = Sha256::new();
        hasher.update(password);
        hasher.update(creation_time.format("%Y-%m-%d-%H-%M").to_string());
        hasher.update(num.to_string());
        Ok(format!("{:x}", hasher.finalize()))
    }

    pub fn check_login(&self, username : &str, password : &str) -> bool{
        let saved_password_hash : Result<Rc<str>> = self.conn.query_row(
            "SELECT password_hash, name FROM person WHERE name=?1",
            [username],
            |row| row.get::<usize, Rc<str>>(0),
        );
        if let Err(_) = saved_password_hash {
            return false;
        }

        *saved_password_hash.unwrap() == *password

    }

}
