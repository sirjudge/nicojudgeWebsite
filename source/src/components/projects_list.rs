use dioxus::{
    logger::tracing::{debug, error, info, warn, Level},
    prelude::*,
};
use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION, USER_AGENT};
use serde::{Deserialize, Serialize};

#[component]
pub fn ProjectTable() -> Element {
    rsx! {
        div {
            class: "project-table",
            h2 { "Projects" }

            table {
                thead {
                    tr {
                        th { "Name" }
                        th { "Description" }
                        th { "Last Updated" }
                    }
                }
                ProjectsList {}
            }
        }
    }
}

#[component]
pub fn ProjectsList() -> Element {
    let repos = use_signal(|| Vec::<Repository>::new());
    let current_repos = use_resource(move || async move { fetch_github_repos().await });
    rsx! {
        match &*current_repos.read() {
            Some(Ok(repos)) => rsx!{
                tbody {
                    for repo in repos.iter() {
                        tr {
                            // class: "repo_item"
                            td { "{repo.name}" }
                            if let Some(desc) = &repo.description {
                                td { "{desc}" }
                            }
                            else {
                                td { "No description available" }
                            }
                            td { "{repo.pushed_at}" }
                        }
                    }
                }
            },
            Some(Err(e)) => rsx! { p{ "error:{e}"}},
            None => rsx! { p { "Loading . . ."}}
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Repository {
    name: String,
    description: Option<String>,
    pushed_at: String,
}

#[server]
pub async fn fetch_github_repos() -> Result<Vec<Repository>, ServerFnError> {
    let client = reqwest::Client::new();
    let mut headers = HeaderMap::new();
    headers.insert(USER_AGENT, HeaderValue::from_static("rust-app"));
    headers.insert(
        AUTHORIZATION,
        HeaderValue::from_str(&format!("Bearer {}", std::env::var("GITHUB_TOKEN")?))?,
    );

    let repos: Vec<Repository> = client
        .get("https://api.github.com/user/repos?sort=pushed&direction=desc")
        .headers(headers)
        .send()
        .await?
        .json()
        .await?;

    println!("Fetched {} repositories", repos.len());
    println!("First repository: {:?}", repos.first());
    Ok(repos)
}
