use crate::{components::errors::UnexpectedError, models::Repository};
use dioxus::{prelude::*, logger::tracing::error};
use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION, USER_AGENT};

#[component]
pub fn ProjectTable() -> Element {
    rsx! {
        div {
            class: "projects-table",
            h2 { "Projects" }
            table {
                thead {
                    tr {
                        th { "Name" }
                        th { "Description" }
                        th { "Last Updated" }
                    }
                }
                ProjectsTableBody {}
            }
        }
    }
}

#[component]
pub fn ProjectsTableBody() -> Element {
    let repo_list = use_resource(move || async move { fetch_github_repos().await });
    rsx! {
        match &*repo_list.read() {
            Some(Ok(repos)) => rsx!{
                tbody {
                    for repo in repos.iter() {
                        tr {
                            class: "repo-row",
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
            Some(Err(e)) => {
                // UnexpectedError()
                rsx! {
                    p { "Error: {e}" }
                }
            },
            None => rsx! { p { "Loading . . ."}}
        }
    }
}

#[server]
pub async fn fetch_github_repos() -> Result<Vec<Repository>, ServerFnError> {
    //TODO: Should maybe think about declaring the client outside of this function and having it
    //accessible instead of re-initializing it every time this function is called.
    let client = reqwest::Client::new();
    let mut headers = HeaderMap::new();
    // TODO: Just noticed I have this user_agent that makes very little sense, don't think I need
    // it so commenting out until verify I actually need it
    // headers.insert(USER_AGENT, HeaderValue::from_static("rust-app"));
    headers.insert(
        AUTHORIZATION,
        HeaderValue::from_str(&format!("Bearer {}", std::env::var("GITHUB_TOKEN")?))?,
    );

    let git_api_url = "https://api.github.com/user/repos?sort=pushed&direction=desc";

    match client
        .get(git_api_url)
        .headers(headers)
        .send()
        .await {
            Ok(response) => {
                if response.status().is_success() {
                    match response.json::<Vec<Repository>>().await {
                        Ok(repos) => Ok(repos),
                        Err(e) => Err(ServerFnError::new(e.to_string())),
                    }
                }
                else {
                    error!("Failed to fetch repositories: {}", response.status());
                    Err(ServerFnError::new(format!("Failed to fetch repositories: {}",response.status())))
                }
            },
            Err(e) => Err(ServerFnError::new(e.to_string())),
    }
}
