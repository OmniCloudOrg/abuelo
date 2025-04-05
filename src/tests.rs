#[cfg(test)]
mod unit_tests {
    use crate::database::{Database, UserCreationError};
    use crate::handle::Handle;
    use std::fs;
    use std::path::Path;

    fn setup_test_db() -> Database {
        // Remove the test database if it exists
        if Path::new("test_user_db.db3").exists() {
            fs::remove_file("test_user_db.db3").expect("Failed to remove test database");
        }
        
        Database::new_with_path("test_user_db.db3")
    }

    #[test]
    fn test_database_user_creation() {
        let db = setup_test_db();
        
        // Test creating a new user
        let result = db.add_user("test_user", "test_password");
        assert!(result.is_ok());
        
        // Test creating a duplicate user
        let result = db.add_user("test_user", "another_password");
        assert!(matches!(result, Err(UserCreationError::UsernameTaken)));
    }

    #[test]
    fn test_user_authentication() {
        let db = setup_test_db();
        
        // Create a user first
        let _ = db.add_user("auth_test_user", "test_password");
        
        // Test correct authentication
        assert!(db.check_login("auth_test_user", "test_password"));
        
        // Test incorrect password
        assert!(!db.check_login("auth_test_user", "wrong_password"));
        
        // Test non-existent user
        assert!(!db.check_login("nonexistent_user", "any_password"));
    }

    #[test]
    fn test_user_retrieval() {
        let db = setup_test_db();
        
        // Create a user first
        let _ = db.add_user("get_test_user", "test_password");
        
        // Test getting an existing user
        let result = db.get_user("get_test_user");
        assert!(result.is_ok());
        
        let user = result.unwrap();
        assert_eq!(user.username(), "get_test_user");
        assert!(!user.premium());
        
        // Test getting a non-existent user
        let result = db.get_user("nonexistent_user");
        assert!(result.is_err());
    }

    #[test]
    fn test_handle_creation() {
        let db = setup_test_db();
        
        // Create a user first
        let _ = db.add_user("handle_test_user", "test_password");
        let user = db.get_user("handle_test_user").unwrap();
        
        // Test creating a handle
        let handle_result = Handle::new(&user, db);
        assert!(handle_result.is_ok());
        
        let handle = handle_result.unwrap();
        assert!(handle.get() > 0);
    }
}