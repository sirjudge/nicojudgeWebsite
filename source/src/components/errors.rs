use dioxus::prelude::*;

#[component]
pub fn UnexpectedError() -> Element {
    //TODO: Should accept the error as a property and log the error via console
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
    //TODO: Should accept the resource attempted to be reached as a property and log the error via console
    rsx! {
        div {
            class: "resouce-not-found",
            h1 { "Error" }
            p { "the resource you're looking for could not be found" }
        }
    }
}
