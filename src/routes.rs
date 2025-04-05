use chrono::{DateTime, Utc};
use rocket::post;


use rocket::{serde::json::Json, Route};
use rocket::get;
use rocket::routes as rocket_routes;
use crate::{database::Database, handle::Handle};

pub fn get_routes() -> Vec<Route> {
    rocket_routes![create_user, auth_user, get_user]
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct UserCreateRequest {
    username: String,
    password: String,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct UserCreateResponse {
    success: bool,
    message: String,
}

#[post("/user/create", data = "<body>")]
fn create_user(body: Json<UserCreateRequest>) -> Json<UserCreateResponse> {
    let db = Database::new();
    let result = db.add_user(&body.username, &body.password);
    let reply = if result.is_ok() {
        UserCreateResponse {
            success: true,
            message: "".to_string(),
        }
    } else {
        UserCreateResponse {
            success: false,
            message: format!("{:#?}", result.unwrap_err()),
        }
    };
    Json(reply)
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct UserGetResponse {
    success: bool,
    message: String,
    creation_time: Option<DateTime<Utc>>,
    premium: Option<bool>,
}

#[get("/user/<username>")]
fn get_user(username: String) -> Json<UserGetResponse> {
    log::info!("Got {} user.", username);
    let db = Database::new();
    let acc = db.get_user(&username);
    let reply = if acc.is_err() {
        UserGetResponse {
            success: false,
            message: format!("{}", acc.unwrap_err()),
            creation_time: None,
            premium: None,
        }
    } else {
        let acc = acc.unwrap();
        UserGetResponse {
            success: true, // Fixed this to be true when successful
            message: "".to_string(),
            creation_time: Some(acc.creation_time()),
            premium: Some(acc.premium()),
        }
    };
    Json(reply)
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct UserAuthRequest {
    username: String,
    password: String,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct UserAuthResponse {
    success: bool,
    handle: Option<u64>,
    message: String,
}

#[post("/user/auth", data = "<body>")]
fn auth_user(body: Json<UserAuthRequest>) -> Json<UserAuthResponse> {
    log::info!("Authing user rn.");
    let db = Database::new();
    let reply = if db.check_login(&body.username, &body.password) {
        // TODO: get rid of unwraps here in favor of good responses
        let inner_handle = db.get_user(&body.username).unwrap();
        let handle = Handle::new(&inner_handle, db).unwrap();
        log::info!("All good");
        let handle2 = Some(handle.get());
        UserAuthResponse {
            success: true,
            message: "".to_string(),
            handle: handle2,
        }
    } else {
        log::info!("Failed to auth user {}", body.username);
        UserAuthResponse {
            success: false,
            message: "Username or Password is invalid".to_string(),
            handle: None,
        }
    };
    Json(reply)
}