use dioxus::prelude::*;
use crate::components::{NewEditBlog, MaintenanceSettings};

#[component]
pub fn AdminView() -> Element {
    // TODO: check to see if the user is verifiedly logged in, maybe do some
    // cookie magic or session management stuff
    rsx! {
        div {
            class: "admin-page",
            h1 { "Admin Page" }
            p { "This is the admin page for managing the application." }
            MaintenanceSettings {}
            NewEditBlog {}
        }
    }
}

#[component]
pub fn AdminLogin() -> Element {
    rsx! {
        div {
            class: "admin-login",
            h1 { "Admin Login" }
            form {
                label { "Username:" }
                br { }
                input {
                    r#type: "text",
                    placeholder: "Username",
                    name: "username",
                    required: true,
                }
                br { }
                br { }
                label { "Password:" }
                br { }
                input {
                    r#type: "password",
                    placeholder: "Password",
                    name: "password",
                    required: true,
                }
                br { }
                //TODO: pass this to a server function and validate the login
                // and redirect to admin page if successful
                button {
                    r#type: "submit",
                    "Login"
                }
            }
        }
    }
}
