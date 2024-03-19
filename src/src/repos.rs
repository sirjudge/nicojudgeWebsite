use leptos::*;
use serde::{Deserialize, Serialize};

struct repo {
    name: String,
    html_url: String
}

fn get_repos() -> Vec<repo> {
    //let url = "https://api.github.com/users/nicojudge/repos";
    // hardcode a list for now
    let repos = vec![
        repo {
            name: "nico-website".to_string(),
            html_url: "".to_string()
        },
    ];

    /*c
    let request = Request::new(url);
    let git_auth_token = "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";
    let auth_token = format!("bearer {}", git_auth_token );
    request.headers().set("Authorization", auth_token);
    let future = fetch(request)
        .and_then(|response| response.json::<Vec<repo>>())
        .map_err(|_| ());
    future
    */
    repos
}

#[component]
pub fn Repos() -> impl IntoView {
    let repos = get_repos();
    println!("repos: {:?}", repos.len());
    if repos.is_empty() {
        view! {
            <ReposNotLoaded/>
        }
    } else {
        view! {
            <ReposLoaded repos={repos}/>
        }
    }
}

#[component]
fn ReposNotLoaded() -> impl IntoView {
    view! {
        <div id="repos">
            <h1>"Loading Repos..."</h1>
        </div>
    }
}

#[component]
fn ReposLoaded(repos: Vec<repo>) -> impl IntoView {
    view! {
        <div id="repos">
            <h1>"These are my repos!"</h1>
            <p>
                These are my repos 
            </p> 
            <ul>
            </ul>
        </div>
    }
}
