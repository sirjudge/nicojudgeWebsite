use dioxus::{
    logger::tracing::{debug, error, info, warn, Level},
    prelude::*,
};

#[component]
pub fn UnexpectedError() -> Element {
    rsx! {
        div {
            class: "error-page",
            h1 { "Error" }
            p { "An unexpected error has occurred." }
            p { "Please try again later or contact support if the issue persists." }
        }
    }
}

#[component]
pub fn ResourceNotFound() -> Element {
    rsx! {
        div {
            class: "resouce-not-found",
            h1 { "Error" }
            p { "the resource you're looking for could not be found" }
        }
    }
}
