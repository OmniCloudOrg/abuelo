#[macro_use] extern crate rocket;

pub mod account;
pub mod database;
pub mod handle;
/// Module for handling logging functionality
pub mod logger;
pub mod routes;

// #[cfg(test)]
// mod tests;

// Re-export what's needed for the integration tests
pub use routes::get_routes;