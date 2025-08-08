use dioxus::{
    logger::{
        self,
        tracing::{debug, warn, Level},
    },
    prelude::*,
};
use web::{components::MaintenanceBanner, route::Route, views::Navbar};

// Import server functions to ensure they are registered
#[allow(unused_imports)]
use web::auth::{
    login_with_session_and_cookies, logout_with_cookies,
    get_current_user, hash_password, verify_password_hash, validate_login,
};

// We can import assets in dioxus with the `asset!` macro. This macro takes a path to an asset relative to the crate root.
// The macro returns an `Asset` type that will display as the path to the asset in the browser or a local path in desktop bundles.
const FAVICON: Asset = asset!("/assets/favicon.ico");
// The asset macro also minifies some assets like CSS and JS to make bundled smaller
const MAIN_CSS: Asset = asset!("/assets/styling/main.css");
const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");
// Do something better with this but for now fine with keeping it
// as a constant
const MAINTENANCE_MODE: bool = false;

fn main() {
    // Initialize logging
    match logger::init(Level::DEBUG) {
        Ok(_) => {
            debug!("Logger initialized successfully");
        }
        Err(e) => {
            eprintln!("Failed to initialize logger: {e}");
        }
    }

    // Database initialization will be done lazily on first connection
    // This avoids needing tokio in the main function
    #[cfg(feature = "server")]
    {
        use dioxus::logger::tracing::info;
        info!("Database will be initialized on first connection");
    }

    // Use the fullstack launch for Dioxus 0.6 which automatically handles server functions
    dioxus::launch(app);
}

fn app() -> Element {
    if MAINTENANCE_MODE {
        warn!("Maintenance mode is enabled. The site will not be accessible.");
        return rsx! { MaintenanceBanner {} };
    }

    rsx! {
        document::Link { rel: "icon", href: FAVICON },
        document::Link { rel: "stylesheet", href: MAIN_CSS },
        document::Link { rel: "stylesheet", href: TAILWIND_CSS },
        Router::<Route> {}
    }
}
