/// Define a components module that contains all shared components for our app.
pub mod components;

/// Define a views module that contains the UI for all Layouts and Routes for our app.
pub mod views;

/// The models module contains all the data structures and logic for our app.
pub mod models;

/// Database connection and migration management using SQLx.
/// Only included for server builds.
#[cfg(feature = "server")]
pub mod database;

pub mod route;
