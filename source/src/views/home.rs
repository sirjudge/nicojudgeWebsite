use crate::components::{Bio, MaintenanceBanner};
use dioxus::prelude::*;

const MAINTENANCE_MODE: bool = true; // This can be set based on your app's state

/// The Home page component that will be rendered when the current route is `[Route::Home]`
#[component]
pub fn Home() -> Element {
    rsx! {
        Bio {}
    }
}
