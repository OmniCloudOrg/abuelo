use chrono::{DateTime, Utc};

pub type UserID = u128;

pub enum AccountStatusState {
    Offline,
    Away,
    DoNotDisturb,
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
