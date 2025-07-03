use dioxus::prelude::*;

#[component]
pub fn ProjectsList() -> Element {
    // TODO: Need to get a list of my github repositories and display them here
    // based on whatever I've done recently
    rsx! {
        div {
            class: "projects",
            h2 { "Projects" }
            p { "Here are some of my recent projects:" }
            ul {
                li { "Project 1: A web application built with Rust and Dioxus." }
                li { "Project 2: A command-line tool for data processing." }
                li { "Project 3: An open-source library for Rust developers." }
            }
        }
    }
}
