use chrono::{DateTime, Utc};

pub type UserID = u128;

pub enum AccountStatusState {
    Offline,
    Idle,
    // Similar to discords do not disturb.
    Silenced,
    Online,
}

pub struct AccountStatus{
    state: AccountStatusState,
    tagline: String,
}


pub struct Account{
    username: String,
    user_id: UserID,
    password_hash: String,
    status: AccountStatus,
    created_date: DateTime<Utc>,
    // Donator role
    premium: bool,
}
