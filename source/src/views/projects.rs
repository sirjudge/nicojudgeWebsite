use crate::components::ProjectTable;
use crate::views::Navbar;
use dioxus::prelude::*;

#[component]
pub fn Projects() -> Element {
    rsx! {
        Navbar {}
        ProjectTable {}
    }
}
