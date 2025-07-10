use dioxus::{
    dioxus_core::LaunchConfig,
    logger::{
        self,
        tracing::{debug, error, field::debug, warn, Level},
    },
    prelude::*,
    web::Config,
};
use web::{components::MaintenanceBanner, route::Route};

// Server-only imports for Axum integration
#[cfg(feature = "server")]
use dioxus::fullstack::prelude::ServeConfigBuilder;

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

    let addr = dioxus_cli_config::fullstack_address_or_localhost();
    debug!("Starting Dioxus app at {addr}");

    // Configure the server to serve static assets
    #[cfg(feature = "server")]
    let config = ServeConfigBuilder::default().build().unwrap();

    #[cfg(feature = "server")]
    dioxus::LaunchBuilder::new().with_cfg(config).launch(app);

    #[cfg(not(feature = "server"))]
    dioxus::LaunchBuilder::new().launch(app);

    LaunchBuilder::new()
        // Only set the server config if the server feature is enabled
        .with_cfg(server_only! {
            ServeConfigBuilder::default()
                .root_id("app")
        })
        // You also need to set the root id in your web config
        .with_cfg(web! {
            dioxus::web::Config::default().rootname("app")
        })
        //NOTE: Don't need this as we aren't serving a desktop application at
        //the moment, it just be fullstack/server + web
        // And desktop config
        // .with_cfg(desktop! {
        //     dioxus::desktop::Config::default().with_root_name("app")
        // })
        .launch(app);
}

fn app() -> Element {
    if MAINTENANCE_MODE {
        error!("Maintenance mode is enabled. The site will not be accessible.");
        return rsx! { MaintenanceBanner {} };
    }

    rsx! {
        div {
            document::Link { rel: "icon", href: FAVICON },
            document::Link { rel: "stylesheet", href: MAIN_CSS },
            document::Link { rel: "stylesheet", href: TAILWIND_CSS },
            Router::<Route> {}
        }
    }
}
