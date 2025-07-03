use dioxus::prelude::*;

#[component]
pub fn MaintenanceBanner() -> Element {
    rsx! {
        div {
            class: "maintenance-banner",
            h2 { "Maintenance Mode" }
            p { "The site is currently undergoing maintenance. Please check back later." }
            p { "Thank you for your patience!" }
        }
    }
}
