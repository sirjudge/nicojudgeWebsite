use dioxus::logger::tracing::info;
use dioxus::{html::input, prelude::*};

#[component]
pub fn MaintenanceSettings() -> Element {
    let mut maintenance_box = use_signal(|| false);
    rsx! {
        div {
            class: "maintenance-mode",
            h1 { "Maintenance Mode" }
            p { "The site is currently undergoing maintenance. Please check back later." }
            // You can add more details or a contact link here
            form {
                onsubmit:  move |_| {
                    if *maintenance_box.read() {
                        info!("enbabling maintenance_mode");
                    }
                    else {
                        info!("Disabling maintenance_mode");
                    }
                },
                // Check box to turn site on and off of maintenance mode
                input {
                    r#type: "checkbox",
                    name: "maintenance_mode",
                    id: "maintenance_mode",
                    oninput: move |input_event| {
                        info!("maintenance_mode checkbox:{}",input_event.value());
                        maintenance_box.set(input_event.value() == "true");
                    }
                }
                button {
                    r#type: "submit",
                    "update maintenance_mode"
                }
            }
        }
    }
}

// #[server]
// async fn save_mode(enabled: bool) -> Result<(), ServerFnError> {}
