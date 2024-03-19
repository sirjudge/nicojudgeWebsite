use leptos::*;
use std::env;

struct repo {
    name: String,
    html_url: String,
}

#[component]
pub fn Repos() -> impl IntoView {
    let repos = get_repos();
    if repos.is_empty() {
        view! {
        
        }
    } 
    else {
        view! {
        }

    }
}

fn get_repos() -> Vec<repo> {
   let repos = vec![
        repo {
            name: "leptos".to_string(),
            html_url: "www.github.com/sirjudge".to_string)
    /* 
    let url = "https://api.github.com/users/nicojudge/repos";
    let request = Request::new(url);
    let auth_token = format!("bearer {}",std::env!("GITHUB_AUTH_TOKEN"));
    request.headers().set("Authorization", auth_token);
    let future = fetch(request)
        .and_then(|response| response.json::<Vec<repo>>())
        .map_err(|_| ());
    let repos = futures::executor::block_on(future);
  */ 
    repos
}

#[component]
fn reposNotLoaded() -> impl IntoView {
    view! {
        <div id="repos">
            <h1>"Loading Repos..."</h1>
        </div>
    }
}

#[Component]
fn ReposLoaded(repos: Vec<repo>) -> impl IntoView {
    view! {
        <div id="repos">
            <h1>"These are my repos!"</h1>
            <p>
                These are my repos 
            </p> 
            <ul>
                {for repos.iter().map(|repo| {
                    view! {
                        <li>
                            <a href={repo.html_url.clone()}>{repo.name.clone()}</a>
                        </li>
                    }
                })}
            </ul>
        </div>
    }
}
