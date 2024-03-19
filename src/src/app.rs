use leptos::*;
use leptos_meta::*;
use leptos_router::*;
use crate::input;

/// Entry point for the application
#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();
    view! {
        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/nico-website.css"/>
        // content for this welcome page
        <Header/>
        <Router>
            <main>
                <Routes>
                    <Route path="" view=HomePage/>
                    <Route path="/*any" view=NotFound/>
                </Routes>
            </main>
        </Router>
    }
}

/// Renders the home page of your application.
#[component]
fn HomePage() -> impl IntoView {
    view! {
        <h1>"Welcome!"</h1>
        <p>
            Welcome to my website! This website is powered by Leptos, a Rust 
            web framework that uses WebAssembly and server-side rendering to
            create fast, modern web applications and is deplyoyed and ran on 
            a docker file hosted on a linux server.
        </p>
        <input::Input/>
    }
}

/// Renders the headers of the application
#[component]
fn Header() -> impl IntoView {
    view! {
        <Title text="nicojudge.com"/>
        <header>
            <h1>"Nico Judge"</h1>
            <h1>"Full Stack Software Engineer"</h1>
        </header>
    }
}

/// 404 - Not Found
#[component]
fn NotFound() -> impl IntoView {
    // set an HTTP status code 404
    // this is feature gated because it can only be done during
    // initial server-side rendering
    // if you navigate to the 404 page subsequently, the status
    // code will not be set because there is not a new HTTP request
    // to the server
    #[cfg(feature = "ssr")]
    {
        // this can be done inline because it's synchronous
        // if it were async, we'd use a server function
        let resp = expect_context::<leptos_actix::ResponseOptions>();
        resp.set_status(actix_web::http::StatusCode::NOT_FOUND);
    }

    view! {
        <h1>"Not Found"</h1>
    }
}
