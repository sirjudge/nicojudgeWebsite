use dioxus::html::image;
use dioxus::logger::tracing::{debug, error, info, warn};
use dioxus::prelude::*;

const NICO_MOWER_PIC: Asset = asset!(
    "/assets/images/nico_mower.jpg",
    ImageAssetOptions::new()
    .with_format(ImageFormat::Avif)
    .with_size(ImageSize::Automatic)
);

#[component]
pub fn Bio() -> Element {
    rsx! {
        div {
            class: "bio",
            style: "display: flex; flex-direction: column;",
            h2 { 
                style: "margin-bottom: 20px;",
                "About Me" 
            },
            // Two-column content area
            div {
                style: "display: flex; flex-direction: row; gap: 20px; align-items: flex-start;",
                // First column: Image
                div {
                    style: "flex: 0 0 auto;",
                    img {
                        id: "bio_main_pic",
                        src: NICO_MOWER_PIC,
                        style: "display: block; max-width: 300px; max-height: 300px; width: auto; height: auto;"
                    }
                },
                // Second column: Text content
                div {
                    style: "flex: 1;",
                    p {
                        style: "margin-bottom: 15px;",
                        "Hello! I'm a fullstack software engineer with nearly a decade in
                        building high performant web applications, services, and tooling for
                        nearly a decade."
                    },
                    p {
                        "The page you're viewing right now is built from
                        the ground up with Rust and Dioxus, a modern web framework that
                        allows for building fast and efficient web applications with
                        the power of WASM"
                    }
                }
            }
        }
    }
}
