use chrono::{DateTime, Utc};
use rocket::post;
#[derive(serde::Serialize, serde::Deserialize)]
pub struct UserHandlesResponse {
    success: bool,
    message: String,
    handles: Option<Vec<u64>>,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct HandleRequest {
    username: String,
    password: String,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct HandleResponse {
    success: bool,
    message: String,
    handle: Option<u64>,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct DeleteHandleRequest {
    username: String,
    password: String,
    handle: u64,
}

#[get("/user/<username>/handles")]
fn get_user_handles(username: String) -> Json<UserHandlesResponse> {
    log::info!("Getting handles for user: {}", username);
    let db = Database::new();
    
    let user_result = db.get_user(&username);
    if user_result.is_err() {
        let err = user_result.err().unwrap();
        log::error!("User not found while getting handles: {}, error: {:?}", username, err);
        return Json(UserHandlesResponse {
            success: false,
            message: format!("User not found: {}", username),
            handles: None,
        });
    }
    
    let user = user_result.unwrap();
    let handles_result = Handle::get_all_for_user(user.id(), &db);
    
    match handles_result {
        Ok(handles) => {
            let handle_values: Vec<u64> = handles.iter().map(|h| h.get()).collect();
            log::info!("Successfully retrieved {} handles for user: {}", handle_values.len(), username);
            Json(UserHandlesResponse {
                success: true,
                message: "".to_string(),
                handles: Some(handle_values),
            })
        },
        Err(err) => {
            log::error!("Error retrieving handles for user {}: {:?}", username, err);
            Json(UserHandlesResponse {
                success: false,
                message: format!("Error retrieving handles: {}", err),
                handles: None,
            })
        },
    }
}

#[post("/user/handle/create", data = "<body>")]
fn create_new_handle(body: Json<HandleRequest>) -> Json<HandleResponse> {
    log::info!("Creating new handle for user: {}", body.username);
    let db = Database::new();
    
    if !db.check_login(&body.username, &body.password) {
        log::warn!("Authentication failed during handle creation for user: {}", body.username);
        return Json(HandleResponse {
            success: false,
            message: "Invalid username or password".to_string(),
            handle: None,
        });
    }
    
    let user_result = db.get_user(&body.username);
    if user_result.is_err() {
        let err = user_result.err().unwrap();
        log::error!("User not found during handle creation: {}, error: {:?}", body.username, err);
        return Json(HandleResponse {
            success: false,
            message: "User not found".to_string(),
            handle: None,
        });
    }
    
    let user = user_result.unwrap();
    match Handle::new(&user, &db) {
        Ok(handle) => {
            log::info!("Successfully created new handle {} for user: {}", handle.get(), body.username);
            Json(HandleResponse {
                success: true,
                message: "Handle created successfully".to_string(),
                handle: Some(handle.get()),
            })
        },
        Err(err) => {
            log::error!("Failed to create handle for user {}: {:?}", body.username, err);
            Json(HandleResponse {
                success: false,
                message: format!("Failed to create handle: {:?}", err),
                handle: None,
            })
        },
    }
}

#[post("/user/handle/delete", data = "<body>")]
fn delete_handle(body: Json<DeleteHandleRequest>) -> Json<HandleResponse> {
    log::info!("Deleting handle {} for user: {}", body.handle, body.username);
    let db = Database::new();
    
    if !db.check_login(&body.username, &body.password) {
        log::warn!("Authentication failed during handle deletion for user: {}", body.username);
        return Json(HandleResponse {
            success: false,
            message: "Invalid username or password".to_string(),
            handle: None,
        });
    }
    
    let user_result = db.get_user(&body.username);
    if user_result.is_err() {
        let err = user_result.err().unwrap();
        log::error!("User not found during handle deletion: {}, error: {:?}", body.username, err);
        return Json(HandleResponse {
            success: false,
            message: "User not found".to_string(),
            handle: None,
        });
    }
    
    let user = user_result.unwrap();
    let handle = Handle::from_value(body.handle);
    
    if !handle.is_owned_by_user(user.id(), &db) {
        log::warn!("Attempt to delete handle {} that doesn't belong to user: {}", body.handle, body.username);
        return Json(HandleResponse {
            success: false,
            message: "This handle does not belong to the user".to_string(),
            handle: None,
        });
    }
    
    match handle.delete(user.id(), &db) {
        Ok(true) => {
            log::info!("Successfully deleted handle {} for user: {}", body.handle, body.username);
            Json(HandleResponse {
                success: true,
                message: "Handle deleted successfully".to_string(),
                handle: Some(handle.get()),
            })
        },
        Ok(false) => {
            log::warn!("Handle {} not found during deletion for user: {}", body.handle, body.username);
            Json(HandleResponse {
                success: false,
                message: "Handle not found".to_string(),
                handle: None,
            })
        },
        Err(err) => {
            log::error!("Error deleting handle {} for user {}: {:?}", body.handle, body.username, err);
            Json(HandleResponse {
                success: false,
                message: format!("Error deleting handle: {}", err),
                handle: None,
            })
        },
    }

}
use rocket::{serde::json::Json, Route};


use rocket::get;
use rocket::routes as rocket_routes;
use crate::{database::Database, handle::Handle};

pub fn get_routes() -> Vec<Route> {
    use rocket::routes;
    // rocket_routes![create_user, auth_user, get_user]
    routes![create_user, auth_user, get_user, get_user_handles, create_new_handle, delete_handle]
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
        let handle = Handle::new(&inner_handle, &db).unwrap();
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