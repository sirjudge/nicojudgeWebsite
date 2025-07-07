use crate::components::ProjectTable;
use dioxus::prelude::*;

#[component]
pub fn Projects() -> Element {
    rsx! {
        ProjectTable {}
    }
}
