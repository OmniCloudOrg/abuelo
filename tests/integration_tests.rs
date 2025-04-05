#[cfg(test)]
mod integration_tests {
    use rocket::{http::Status, local::blocking::Client};
    use serde_json::{json, Value};
    use std::fs;
    use std::path::Path;

    fn setup() -> Client {
        // Remove the test database if it exists
        if Path::new("test_user_db.db3").exists() {
            fs::remove_file("test_user_db.db3").expect("Failed to remove test database");
        }

        // Create a Rocket instance for testing
        let rocket = rocket::build()
            .mount("/", abuelo::routes::get_routes())
            .configure(rocket::Config {
                // Set test-specific configuration
                address: std::net::Ipv4Addr::new(127, 0, 0, 1).into(),
                port: 0, // Use a random port for testing
                log_level: rocket::config::LogLevel::Debug,
                ..rocket::Config::debug_default()
            });

        // Create a client to send requests to our Rocket instance
        Client::tracked(rocket).expect("Valid rocket instance")
    }

    #[test]
    fn test_create_user() {
        let client = setup();

        // Test user creation
        let response = client.post("/user/create")
            .json(&json!({
                "username": "test_user",
                "password": "test_password"
            }))
            .dispatch();

        assert_eq!(response.status(), Status::Ok);
        
        let body: Value = serde_json::from_str(&response.into_string().unwrap()).unwrap();
        assert_eq!(body["success"], true);
        assert_eq!(body["message"], "");

        // Test creating the same user again (should fail)
        let response = client.post("/user/create")
            .json(&json!({
                "username": "test_user",
                "password": "test_password"
            }))
            .dispatch();

        assert_eq!(response.status(), Status::Ok);
        
        let body: Value = serde_json::from_str(&response.into_string().unwrap()).unwrap();
        assert_eq!(body["success"], false);
        assert!(body["message"].as_str().unwrap().contains("Username was taken"));
    }

    #[test]
    fn test_user_auth() {
        let client = setup();

        // Create a user first
        client.post("/user/create")
            .json(&json!({
                "username": "auth_test_user",
                "password": "auth_test_password"
            }))
            .dispatch();

        // Test successful authentication
        let response = client.post("/user/auth")
            .json(&json!({
                "username": "auth_test_user",
                "password": "auth_test_password"
            }))
            .dispatch();

        assert_eq!(response.status(), Status::Ok);
        
        let body: Value = serde_json::from_str(&response.into_string().unwrap()).unwrap();
        assert_eq!(body["success"], true);
        assert_eq!(body["message"], "");
        assert!(body["handle"].is_u64());

        // Test authentication with wrong password
        let response = client.post("/user/auth")
            .json(&json!({
                "username": "auth_test_user",
                "password": "wrong_password"
            }))
            .dispatch();

        assert_eq!(response.status(), Status::Ok);
        
        let body: Value = serde_json::from_str(&response.into_string().unwrap()).unwrap();
        assert_eq!(body["success"], false);
        assert_eq!(body["message"], "Username or Password is invalid");
        assert!(body["handle"].is_null());
    }

    #[test]
    fn test_get_user() {
        let client = setup();

        // Create a user first
        client.post("/user/create")
            .json(&json!({
                "username": "get_test_user",
                "password": "get_test_password"
            }))
            .dispatch();

        // Test getting user info
        let response = client.get("/user/get_test_user").dispatch();
        assert_eq!(response.status(), Status::Ok);
        
        let body: Value = serde_json::from_str(&response.into_string().unwrap()).unwrap();
        assert_eq!(body["success"], true);
        assert_eq!(body["message"], "");
        assert!(body["creation_time"].is_string());
        assert_eq!(body["premium"], false);

        // Test getting non-existent user
        let response = client.get("/user/nonexistent_user").dispatch();
        assert_eq!(response.status(), Status::Ok);
        
        let body: Value = serde_json::from_str(&response.into_string().unwrap()).unwrap();
        assert_eq!(body["success"], false);
        assert!(body["message"].as_str().unwrap().contains("QueryReturnedNoRows"));
        assert!(body["creation_time"].is_null());
        assert!(body["premium"].is_null());
    }
}