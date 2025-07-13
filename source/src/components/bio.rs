use dioxus::prelude::*;
use dioxus::logger::tracing::{info, warn, debug, error};

#[component]
pub fn Bio() -> Element {
    info!("Loading bio element");
    rsx! {
        div {
            class: "bio",
            h2 { "About Me" }
            p { "
                Hello! I'm a fullstack software engineer with nearly a decade in
                building high performant web applications, services, and tooling for
                nearly a decade. The page you're viewing right now is built from
                the ground up with Rust and Dioxus, a modern web framework that
                allows for building fast and efficient web applications with
                the power of WASM" }
        }
    }
}
