use crate::{
    account::Account,
    database::{Database, HandleDBError},
};

pub struct Handle(u64);

impl Handle {
    pub fn new(user: &Account, db: Database) -> Result<Handle, HandleDBError> {
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
}
