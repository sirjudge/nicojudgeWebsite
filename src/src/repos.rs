use leptos::*;
use std::clone::Clone;
use async_std::task::block_on;
//use http::{Request, StatusCode};
use std::collections::HashMap;
use std::time::Duration;

#[derive(Copy, Clone, Debug, PartialEq,Eq)]
struct Repo {
    name: RwSignal<String>,
    description: RwSignal<String>,
    html_url: RwSignal<String>,
}

/// Root level component for the repos section
#[component]
pub fn Repos() -> impl IntoView {
    let repos = get_repos();
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
    async {
        let request = reqwest::Client::new()
            .get("https://api.github.com/users/sirjudge/repos");
        println!("request created: {:?}", request);
        let response = request.send().await; 
        println!("request send, response recieved");
        match response {
            Ok(response) => {
                let body = response.text().await.unwrap();
                println!("body: {}", body);
            }
            Err(e) => {
                println!("error: {}", e);
            }
        } 
    };

    // generate hardcoded list
    let mut repos = Vec::new();
    let repo1 = Repo {
        name: RwSignal::new("data_comparison_tool".to_string()),
        description: RwSignal::new("Tool used to take in large amounts of data and return added, deleted, and changed rows".to_string()),
        html_url: RwSignal::new("https://github.com/sirjudge/data_comparison_tool".to_string()),
    };
   
    let repo2 = Repo {
        name: RwSignal::new("PartyApp".to_string()),
        description: RwSignal::new("C# application built using Avalonia UI framework and .net core web API to create a simple chat application with sqlite backend to store messages".to_string()),
        html_url: RwSignal::new("https://github.com/sirjudge/PartyApp".to_string()),
    };

    // just hard code for now
    repos.push(repo1);
    repos.push(repo2);
    repos
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
            <h1> "Repos" </h1>
            <For
                // function to return items we're iterating over
                each = move || repo_data.get()
                key = |repo: &Repo| repo.name.get()
                children = move |repo : Repo| {
                    view! {
                        <li> 
                        {repo.name.get()} - {repo.description.get()} - <a href={repo.html_url.get()}> {repo.html_url.get()} </a>   
                        </li>
                    }
                }
            />
        </div>
    }
}
