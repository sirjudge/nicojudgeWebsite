use dioxus::{
    logger::{self, tracing::{debug, error, field::debug, warn, Level}},
    prelude::*,
};
use web::{components::MaintenanceBanner, route::Route};

// // Server-only imports for Axum integration
// #[cfg(feature = "server")]
// use dioxus::fullstack::prelude::ServeConfigBuilder;

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
    match logger::init(Level::DEBUG) {
        Ok(_) => {
            // Logger initialized successfully
            debug!("Logger initialized successfully");
        }
        Err(e) => {
            eprintln!("Failed to initialize logger: {e}");
        }
    }


    dioxus::launch( || {
        if MAINTENANCE_MODE {
            error!("Maintenance mode is enabled. The site will not be accessible.");
            return rsx! { MaintenanceBanner {} };
        }

        rsx! {
            document::Link { rel: "icon", href: FAVICON }
            document::Link { rel: "stylesheet", href: MAIN_CSS }
            document::Link { rel: "stylesheet", href: TAILWIND_CSS }
            Router::<Route> {}
        }
    })
}
