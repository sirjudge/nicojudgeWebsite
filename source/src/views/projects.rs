use crate::components::ProjectsList;
use dioxus::prelude::*;

#[component]
pub fn Projects() -> Element {
    rsx! {
        ProjectsList {}
    }
}
