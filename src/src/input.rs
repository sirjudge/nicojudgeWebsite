use leptos::*;

/// default component to demonstrate signals
#[component]
pub fn Input() -> impl IntoView {
    // Creates a reactive value to update the button
    let (count, set_count) = create_signal(0);
    let on_click = move |_| set_count.update(|count| *count += 1);
    view! {
        <p>
            "This is a simple counter to demonstrate the reactive nature of Leptos"
        </p>
        <button on:click=on_click>"Click Me: " {count}</button>
    }
}
