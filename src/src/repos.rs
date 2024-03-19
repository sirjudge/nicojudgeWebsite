
#[component]
pub fn Repos() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();
    view! {
        <div id="repos">
            <h1>"These are my repos!"</h1>
            <p>
                These are my repos 
            </p> 
        </div>
    }
}
