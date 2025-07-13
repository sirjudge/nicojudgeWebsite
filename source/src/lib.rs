/// Define a components module that contains all shared components for our app.
pub mod components;

/// Define a views module that contains the UI for all Layouts and Routes for our app.
pub mod views;

/// The models module contains all the data structures and logic for our app.
pub mod models;

/// Schema module contains the database schema for our app used by diesel.
/// Only included for server builds.
#[cfg(feature = "server")]
pub mod schema;

pub mod route;
