use crate::models::Repository;
use dioxus::{logger::tracing::error, prelude::*};
#[cfg(feature = "server")]
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
    //TODO: Should really move this client to be initialized in the app or higher up but fine to
    //keep for now
    let client = reqwest::Client::new();

    // Create auth and user agent headers
    let mut headers = HeaderMap::new();
    headers.insert(USER_AGENT, HeaderValue::from_static("nicojudgedotcom"));
    let git_token =std::env::var("GITHUB_TOKEN")?;
    headers.insert(
        AUTHORIZATION,
        HeaderValue::from_str(&format!("Bearer {}",git_token))?,
    );

    //TODO: Should really cache this to prevent spamming attacks of this URL
    //maybe something like this:
    //https://docs.rs/http-cache-reqwest/latest/http_cache_reqwest/
    // initialize git api endpoint
    let git_api_url = "https://api.github.com/user/repos?sort=pushed&direction=desc";

    // return either the repo vector or a prorper error response string
    //BUG: This is failing in the docker container but not when running local
    match client.get(git_api_url).headers(headers).send().await {
        Ok(response) => {
            if response.status().is_success() {
                match response.json::<Vec<Repository>>().await {
                    Ok(repos) => Ok(repos),
                    Err(e) => Err(ServerFnError::new(e.to_string())),
                }
            } else {
                error!("Failed to fetch repositories: {}", response.status());
                Err(ServerFnError::new(format!(
                    "Failed to fetch repositories: {}",
                    response.status()
                )))
            }
        }
        Err(e) => Err(ServerFnError::new(e.to_string())),
    }
}
