use rusqlite::Result;

use crate::{
    account::{Account, UserID},
    database::{Database, HandleDBError},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub struct Handle(u64);

impl Handle {
    pub fn new(user: &Account, db: &Database) -> Result<Handle, HandleDBError> {
        loop {
            let num = rand::random::<u64>();
            let res = db.add_handle_to_db(user, num);
            match res {
                Ok(_) => return Ok(Handle(num)),
                Err(x) => match x {
                    HandleDBError::HandleAlreadyExists => continue,
                    HandleDBError::DBError(e) => return Err(HandleDBError::DBError(e)),
                },
            }
        }
    }

    pub fn get(&self) -> u64 {
        self.0
    }
    
    pub fn from_value(value: u64) -> Self {
        Handle(value)
    }
    
    // Get all handles for a user
    pub fn get_all_for_user(user_id: UserID, db: &Database) -> Result<Vec<Handle>> {
        let handle_values = db.get_handles_for_user(user_id)?;
        let handles = handle_values.into_iter().map(Handle::from_value).collect();
        Ok(handles)
    }
    
    // Check if a handle is owned by a user
    pub fn is_owned_by_user(&self, user_id: UserID, db: &Database) -> bool {
        db.is_handle_owned_by_user(self.0, user_id)
    }
    
    // Delete a handle
    pub fn delete(&self, user_id: UserID, db: &Database) -> Result<bool> {
        db.delete_handle(self.0, user_id)
    }
}