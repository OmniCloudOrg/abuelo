use std::{fmt::Display, rc::Rc};

use chrono::{DateTime, Utc};
use rusqlite::{Connection, Result};
use sha2::{Digest, Sha256};

use crate::account::{Account, UserID};

pub struct Database {
    conn: Connection,
}

#[derive(Debug)]
pub enum UserCreationError {
    UsernameTaken,
    DBError(rusqlite::Error),
}

impl From<rusqlite::Error> for UserCreationError {
    fn from(value: rusqlite::Error) -> Self {
        Self::DBError(value)
    }
}
impl std::error::Error for UserCreationError {}
impl Display for UserCreationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            UserCreationError::UsernameTaken => {
                write!(f, "Username was taken")
            }
            UserCreationError::DBError(e) => {
                write!(f, "DBError: {}", e)
            }
        }
    }
}

impl Database {
    pub fn new() -> Self {
        let conn = Connection::open("user_db.db3").unwrap();

        // If this returns an error; it's prolly cuz the table already exists.
        // That's fine and we can just let it error and the other queries will
        // use the existing table instead
        // NOTE: Look at the other possible errors here
        let _val = conn.execute(
            "CREATE TABLE users (
            user_id           INTEGER PRIMARY KEY,
            username          TINYTEXT NOT NULL,
            password_hash     TINYTEXT NOT NULL,
            creation_time     DATETIME NOT NULL,
            is_premium        BOOL NOT NULL,
            random_value      INTEGER NOT NULL
        )",
            (),
        );

        Self { conn }
    }

    pub fn get_user(&self, username: &str) -> Result<Account> {
        let (user_id, creation_time, premium, random): (UserID, DateTime<Utc>, bool, i64) =
            self.conn.query_row(
                "SELECT username, user_id, creation_time, is_premium, random_value FROM users WHERE username=?1",
                [username],
                |row| {
                    let user_id = row.get(1)?;
                    let creation_time = row.get(2)?;
                    let premium = row.get(3)?;
                    let random = row.get(4)?;
                    Ok((user_id, creation_time, premium, random))
                },
            )?;
        Ok(Account::new(
            username.to_string(),
            user_id,
            creation_time,
            premium,
            random,
        ))
    }

    pub fn add_user(&self, username: &str, password: &str) -> Result<(), UserCreationError> {
        if self.get_user(username).is_ok() {
            return Err(UserCreationError::UsernameTaken);
        }
        let creation_time = Utc::now();
        let num = rand::random::<u64>();
        let password_hash = self.hash_password(password, creation_time, num);
        self.conn.execute(
            "INSERT INTO users (
            username, 
            password_hash, 
            creation_time, 
            is_premium,
            random_value) 
            VALUES (?1, ?2, ?3, ?4, ?5)",
            (username, password_hash, creation_time, false, num),
        )?;
        Ok(())
    }

    fn hash_password(&self, password: &str, creation_time: DateTime<Utc>, num: u64) -> String {
        let mut hasher = Sha256::new();
        hasher.update(password);
        hasher.update(creation_time.format("%Y-%m-%d-%H-%M").to_string());
        hasher.update(num.to_string());
        format!("{:x}", hasher.finalize())
    }

    pub fn check_login(&self, username: &str, password: &str) -> bool {
        let result = self.conn.query_row(
            "SELECT creation_time, random_value, username FROM users WHERE username=?1",
            [username],
            |row| {
                let creation_time = row.get(0)?;
                let num = row.get(1)?;
                Ok((creation_time, num))
            },
        );
        if result.is_err() {
            return false;
        }
        let (creation_time, num) = result.unwrap();
        let saved_password_hash: Result<Rc<str>> = self.conn.query_row(
            "SELECT password_hash, username FROM users WHERE username=?1",
            [username],
            |row| row.get::<usize, Rc<str>>(0),
        );
        if saved_password_hash.is_err() {
            return false;
        }

        *saved_password_hash.unwrap() == self.hash_password(password, creation_time, num)
    }
}
