use dioxus::{logger, logger::tracing::Level, prelude::*};
use web::{
    components::MaintenanceBanner,
    route::Route,
};

// We can import assets in dioxus with the `asset!` macro. This macro takes a path to an asset relative to the crate root.
// The macro returns an `Asset` type that will display as the path to the asset in the browser or a local path in desktop bundles.
const FAVICON: Asset = asset!("/assets/favicon.ico");
// The asset macro also minifies some assets like CSS and JS to make bundled smaller
const MAIN_CSS: Asset = asset!("/assets/styling/main.css");
const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");

fn main() {
    match logger::init(Level::DEBUG) {
        Ok(_) => {
            // Logger initialized successfully
            println!("Logger initialized successfully");
        }
        Err(e) => {
            // Failed to initialize logger
            //TODO: Re-evaluate what to do in this case
            panic!("Failed to initialize logger: {e}");
        }
    }
    // The `launch` function is the main entry point for a dioxus app. It takes a component and renders it with the platform feature
    // you have enabled
    dioxus::launch(App);
}

const MAINTENANCE_MODE: bool = false;

/// Main entry point for the application. Checks for if app is in maintenance mode and renders
/// either the main app or a maintenance banner.
#[component]
fn App() -> Element {
    // if we're in maintenance mode, render a maintenance message instead of the app
    if MAINTENANCE_MODE {
        return rsx! {
           MaintenanceBanner {}
        };
    }

    // The `rsx!` macro lets us define HTML inside of rust. It expands to an Element with all of our HTML inside.
    rsx! {
        // In addition to element and text (which we will see later), rsx can contain other components. In this case,
        // we are using the `document::Link` component to add a link to our favicon and main CSS file into the head of our app.
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        document::Link { rel: "stylesheet", href: TAILWIND_CSS }

        // The router component renders the route enum we defined above. It will handle synchronization of the URL and render
        // the layouts and components for the active route.
        Router::<Route> {}
    }
}
