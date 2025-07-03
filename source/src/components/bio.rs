use dioxus::prelude::*;

#[component]
pub fn Bio() -> Element {
    rsx! {
        div {
            class: "bio",
            h2 { "About Me" }
            p { "Hello! I'm a software developer with a passion for building web applications. I love working with Rust and Dioxus to create fast and efficient user interfaces." }
            p { "In my free time, I enjoy hiking, reading, and exploring new technologies." }
        }
    }
}
