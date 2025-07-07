use dioxus::prelude::*;

#[component]
pub fn MaintenanceSettings() -> Element {
    rsx! {
        div {
            class: "maintenance-mode",
            h1 { "Maintenance Mode" }
            p { "The site is currently undergoing maintenance. Please check back later." }
            // You can add more details or a contact link here
            form {
                // Check box to turn site on and off of maintenance mode
                input {
                    r#type: "checkbox",
                    name: "maintenance_mode",
                    id: "maintenance_mode",
                    // This would typically be bound to a state or context in a real app
                }
                // //TODO: Fix this later, just keeping it as a placeholder
                // action: "admin/toggleMaintenance",
                // method: "get",
                button {
                    r#type: "submit",
                    "Return to Home"
                }
            }
        }
    }
}
