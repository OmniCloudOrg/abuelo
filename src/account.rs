use chrono::{DateTime, Utc};

pub type UserID = u64;

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct Account {
    username: String,
    user_id: UserID,
    creation_time: DateTime<Utc>,
    // Donator role
    premium: bool,
    random: i64,
}

impl Account {
    pub fn new(
        username: String,
        user_id: UserID,
        creation_time: DateTime<Utc>,
        premium: bool,
        random: i64,
    ) -> Self {
        Self {
            username,
            user_id,
            creation_time,
            premium,
            random,
        }
    }

    pub fn premium(&self) -> bool {
        self.premium
    }

    pub fn creation_time(&self) -> DateTime<Utc> {
        self.creation_time
    }
}
