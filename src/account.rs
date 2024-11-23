use chrono::{DateTime, Utc};

pub type UserID = u64;

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct Account {
    username: String,
    user_id: UserID,
    creation_time: DateTime<Utc>,
    // Donator role
    premium: bool,
}

impl Account {
    pub fn new(
        username: String,
        user_id: UserID,
        creation_time: DateTime<Utc>,
        premium: bool,
    ) -> Self {
        Self {
            username,
            user_id,
            creation_time,
            premium,
        }
    }

    pub fn premium(&self) -> bool {
        self.premium
    }

    pub fn creation_time(&self) -> DateTime<Utc> {
        self.creation_time
    }
}
