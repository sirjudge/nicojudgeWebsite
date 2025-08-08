// The dioxus prelude contains a ton of common items used in dioxus apps. It's a good idea to import wherever you
// need dioxus
use dioxus::prelude::*;
use crate::views::{Admin, Blog, BlogTableOfContents, Home, Navbar, Projects};

/// The Route enum is used to define the structure of internal routes in our app. All route enums need to derive
/// the [`Routable`] trait, which provides the necessary methods for the router to work.
///
/// Each variant represents a different URL pattern that can be matched by the router. If that pattern is matched,
/// the components for that route will be rendered.
#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
pub enum Route {
    // Removing layout system entirely to bypass router parsing issue
    #[route("/")]
    Home {},

    #[route("/blogContents")]
    BlogTableOfContents {},

    #[route("/blog/:id")]
    Blog { id: i32 },

    #[route("/projects")]
    Projects {},

    #[route("/admin")]
    Admin {},
}
