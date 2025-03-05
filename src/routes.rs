use chrono::{DateTime, Utc};
use warp::Filter;

use crate::{database::Database, handle::Handle};

pub fn get_routes() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    create_user().or(auth_user()).or(get_user())
}

#[derive(serde::Serialize, serde::Deserialize)]
struct UserCreateRequest {
    username: String,
    password: String,
}
#[derive(serde::Serialize, serde::Deserialize)]
struct UserCreateResponse {
    success: bool,
    message: String,
}
fn create_user() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::post()
        .and(warp::path!("user" / "create"))
        .and(warp::body::json())
        .map(|body: UserCreateRequest| {
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
            warp::reply::json(&reply)
        })
}

#[derive(serde::Serialize, serde::Deserialize)]
struct UserGetResponse {
    success: bool,
    message: String,
    creation_time: Option<DateTime<Utc>>,
    premium: Option<bool>,
}
fn get_user() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::get()
        .and(warp::path!("user" / String))
        .map(|username: String| {
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
                    success: false,
                    message: "".to_string(),
                    creation_time: Some(acc.creation_time()),
                    premium: Some(acc.premium()),
                }
            };
            warp::reply::json(&reply)
        })
}

#[derive(serde::Serialize, serde::Deserialize)]
struct UserAuthRequest {
    username: String,
    password: String,
}
#[derive(serde::Serialize, serde::Deserialize)]
struct UserAuthResponse {
    success: bool,
    handle: Option<u64>,
    message: String,
}
fn auth_user() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::post()
        .and(warp::path!("user" / "auth"))
        .and(warp::body::json())
        .map(|body: UserAuthRequest| {
            let db = Database::new();
            let reply = if db.check_login(&body.username, &body.password) {
                let handle = Handle::new(&(db.get_user(&body.username).unwrap()), db).unwrap();
                UserAuthResponse {
                    success: true,
                    message: "".to_string(),
                    handle: Some(handle.get()),
                }
            } else {
                UserAuthResponse {
                    success: false,
                    message: "Username or Password is invalid".to_string(),
                    handle: None,
                }
            };
            warp::reply::json(&reply)
        })
}
