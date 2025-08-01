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
            style: "display:flex; flex-direction:column;",
            h2 { "About Me" },
            img {
                id: "bio_main_pic",
                src: NICO_MOWER_PIC,
                style: "display:block;max-width:30%;max-height:30%;width:auto;height:auto;"
            },
            p {
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
