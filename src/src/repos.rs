use leptos::*;
use std::clone::Clone;

#[derive(Copy, Clone, Debug, PartialEq,Eq)]
struct Repo {
    name: RwSignal<String>,
    html_url: RwSignal<String>,
}

/// Root level component for the repos section
#[component]
pub fn Repos() -> impl IntoView {
    let repos = get_repos();
    println!("# repos found: {}", repos.len());
    if repos.is_empty() {
        view! {
            <ReposNotLoaded/>            
        }
    } 
    else {
        view! {
            <ReposLoaded repos={repos}/>
        }

    }
}

/// Get repos from github api
fn get_repos() -> Vec<Repo> {
    // generate hardcoded list
    let mut repos = Vec::new();
    let repo1 = Repo {
        name: RwSignal::new("repo1".to_string()),
        html_url: RwSignal::new("github.com/sirjudge/repo1".to_string()),
    };
    let repo2 = Repo {
        name: RwSignal::new("repo2".to_string()),
        html_url: RwSignal::new("github.com/sirjudge/repo2".to_string()),
    };
    repos.push(repo1);
    repos.push(repo2);
    repos

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
}

/// View for when there are no repos loaded yet
#[component]
fn ReposNotLoaded() -> impl IntoView {
    view! {
        <div id="repos">
            <h1>"Loading Repos..."</h1>
        </div>
    }
}

/// View for when repos are loaded
#[component]
fn ReposLoaded(repos: Vec<Repo>) -> impl IntoView {
    //https://book.leptos.dev/view/04_iteration.html
    //https://docs.rs/leptos/latest/leptos/fn.For.html
    // return the view of each repo rendered out
    let (repo_data, set_repo_data) = create_signal::<Vec<Repo>>(repos);
    view! {
        <div>
            <For
                // function to return items we're iterating over
                each = move || repo_data.get()
                key = |repo: &Repo| repo.name.get()
                children = move |repo : Repo| {
                    view! {
                        <li> {repo.name} </li>
                    }
                }
            />
        </div>
    }
}
