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

#[derive(Debug)]
pub enum HandleDBError {
    HandleAlreadyExists,
    DBError(rusqlite::Error),
}

impl From<rusqlite::Error> for UserCreationError {
    fn from(value: rusqlite::Error) -> Self {
        Self::DBError(value)
    }
}

impl From<rusqlite::Error> for HandleDBError {
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

impl std::error::Error for HandleDBError {}
impl Display for HandleDBError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            HandleDBError::HandleAlreadyExists => {
                write!(f, "Handle already exists.")
            }
            HandleDBError::DBError(e) => {
                write!(f, "DBError: {}", e)
            }
        }
    }
}

impl Database {
    pub fn new() -> Self {
        Self::new_with_path("user_db.db3")
    }
    
    pub fn new_with_path(db_path: &str) -> Self {
        let conn = Connection::open(db_path).unwrap();

        // If this returns an error; it's prolly cuz the table already exists.
        // That's fine and we can just let it error and the other queries will
        // use the existing table instead
        // NOTE: Look at the other possible errors here
        let _val = conn.execute(
            "CREATE TABLE user (
            user_id           INTEGER PRIMARY KEY,
            username          TINYTEXT NOT NULL,
            password_hash     TINYTEXT NOT NULL,
            creation_time     DATETIME NOT NULL,
            is_premium        BOOL NOT NULL,
            random_value      INTEGER NOT NULL
        )",
            (),
        );

        let _val = conn.execute(
            "CREATE TABLE handle (
            handle_id           INTEGER PRIMARY KEY,
            handle_val          BIGINT UNSIGNED NOT NULL,
            user_id             INTEGER NOT NULL,
            CONSTRAINT fk_usr_handle FOREIGN KEY (user_id)     
            REFERENCES user (user_id)
        )",
            (),
        );

        Self { conn }
    }

    pub fn get_user(&self, username: &str) -> Result<Account> {
        let (user_id, creation_time, premium, random): (UserID, DateTime<Utc>, bool, i64) =
            self.conn.query_row(
                "SELECT user_id, creation_time, is_premium, random_value FROM user WHERE username=?1",
                [username],
                |row| {
                    let user_id = row.get(0)?;
                    let creation_time = row.get(1)?;
                    let premium = row.get(2)?;
                    let random = row.get(3)?;
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
        let num = rand::random::<i64>(); // Changed to i64 to match schema
        let password_hash = self.hash_password(password, creation_time, num as u64);
        self.conn.execute(
            "INSERT INTO user (
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
            "SELECT creation_time, random_value FROM user WHERE username=?1",
            [username],
            |row| {
                let creation_time = row.get(0)?;
                let num: i64 = row.get(1)?;
                Ok((creation_time, num as u64))
            },
        );
        if result.is_err() {
            return false;
        }
        let (creation_time, num) = result.unwrap();
        let saved_password_hash: Result<Rc<str>> = self.conn.query_row(
            "SELECT password_hash FROM user WHERE username=?1",
            [username],
            |row| row.get::<usize, Rc<str>>(0),
        );
        if saved_password_hash.is_err() {
            return false;
        }

        *saved_password_hash.unwrap() == self.hash_password(password, creation_time, num)
    }

    // HANDLE FUNCTIONS --------------------------------------------------
    pub fn add_handle_to_db(&self, user: &Account, handle: u64) -> Result<(), HandleDBError> {
        let saved_handle = self
            .conn
            .query_row(
                "SELECT handle_val FROM handle WHERE handle_val=?1",
                [handle],
                |row| row.get::<usize, u64>(0),
            )
            .is_ok();
        if saved_handle {
            return Err(HandleDBError::HandleAlreadyExists);
        }
        self.conn.execute(
            "INSERT INTO handle (
            handle_val,
            user_id
            ) 
            VALUES (?1, ?2)",
            (handle, user.id()),
        )?;
        Ok(())
    }
    
    // Get all handles for a user
    pub fn get_handles_for_user(&self, user_id: UserID) -> Result<Vec<u64>> {
        let mut stmt = self.conn.prepare("SELECT handle_val FROM handle WHERE user_id=?1")?;
        let rows = stmt.query_map([user_id], |row| row.get::<usize, u64>(0))?;
        
        let mut handles = Vec::new();
        for handle_result in rows {
            handles.push(handle_result?);
        }
        
        Ok(handles)
    }
    
    // Check if a handle belongs to a user
    pub fn is_handle_owned_by_user(&self, handle: u64, user_id: UserID) -> bool {
        self.conn
            .query_row(
                "SELECT handle_val FROM handle WHERE handle_val=?1 AND user_id=?2",
                [handle, user_id],
                |row| row.get::<usize, u64>(0),
            )
            .is_ok()
    }
    
    // Delete a handle
    pub fn delete_handle(&self, handle: u64, user_id: UserID) -> Result<bool> {
        let rows_affected = self.conn.execute(
            "DELETE FROM handle WHERE handle_val=?1 AND user_id=?2",
            [handle, user_id],
        )?;
        
        Ok(rows_affected > 0)
    }
}