use crate::models::Repository;
use dioxus::prelude::*;
use dioxus::logger::tracing::error;
#[cfg(feature = "server")]
use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION, USER_AGENT};

/// Format a date string from ISO format to day/month/year format
fn format_date(date_str: &str) -> String {
    // Try to parse the ISO date string and format it
    match chrono::DateTime::parse_from_rfc3339(date_str) {
        Ok(datetime) => {
            // Format as day/month/year
            datetime.format("%d/%m/%Y").to_string()
        }
        Err(_) => {
            // If parsing fails, try to parse as GitHub's format (without timezone)
            match chrono::NaiveDateTime::parse_from_str(date_str, "%Y-%m-%dT%H:%M:%SZ") {
                Ok(naive_datetime) => {
                    naive_datetime.format("%d/%m/%Y").to_string()
                }
                Err(_) => {
                    // If all parsing fails, return the original string
                    date_str.to_string()
                }
            }
        }
    }
}

#[component]
pub fn ProjectTable() -> Element {
    let repo_list = use_resource(move || async move { fetch_github_repos().await });
    
    rsx! {
        div {
            class: "projects-table",
            h2 { "Projects" }
            
            match &*repo_list.read() {
                Some(Ok(repos)) => rsx! {
                    table {
                        style: "
                            width: 100%; 
                            border-collapse: collapse; 
                            margin-top: 20px;
                            background-color: #1a1a1a;
                            color: #ffffff;
                        ",
                        thead {
                            style: "
                                background-color: #2a2a2a;
                            ",
                            tr {
                                th { 
                                    style: "
                                        padding: 12px;
                                        text-align: left;
                                        border-bottom: 2px solid #444;
                                        font-weight: bold;
                                    ",
                                    "Name" 
                                }
                                th { 
                                    style: "
                                        padding: 12px;
                                        text-align: left;
                                        border-bottom: 2px solid #444;
                                        font-weight: bold;
                                    ",
                                    "Description" 
                                }
                                th { 
                                    style: "
                                        padding: 12px;
                                        text-align: left;
                                        border-bottom: 2px solid #444;
                                        font-weight: bold;
                                    ",
                                    "Last Updated" 
                                }
                            }
                        }
                        tbody {
                            style: "background-color: #1a1a1a;",
                            for repo in repos.iter() {
                                tr {
                                    class: "repo-row",
                                    style: "border-bottom: 1px solid #333;",
                                    td { 
                                        style: "padding: 12px; border-bottom: 1px solid #333;",
                                        a {
                                            href: "{repo.html_url}",
                                            target: "_blank",
                                            rel: "noopener noreferrer",
                                            style: "color: #4a9eff; text-decoration: none;",
                                            "{repo.name}"
                                        }
                                    }
                                    if let Some(desc) = &repo.description {
                                        td { 
                                            style: "padding: 12px; border-bottom: 1px solid #333;",
                                            "{desc}" 
                                        }
                                    }
                                    else {
                                        td { 
                                            style: "padding: 12px; border-bottom: 1px solid #333; color: #888;",
                                            "No description available" 
                                        }
                                    }
                                    td { 
                                        style: "padding: 12px; border-bottom: 1px solid #333;",
                                        "{format_date(&repo.pushed_at)}" 
                                    }
                                }
                            }
                        }
                    }
                },
                Some(Err(e)) => rsx! {
                    div {
                        style: "color: red; padding: 20px; background: #2a1a1a; border-radius: 5px; margin-top: 20px;",
                        p { "Error loading repositories: {e}" }
                    }
                },
                None => rsx! {
                    div {
                        style: "text-align: center; padding: 40px; color: #888;",
                        p { "Loading repositories..." }
                    }
                }
            }
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
    let git_token = std::env::var("GITHUB_TOKEN")?;
    headers.insert(
        AUTHORIZATION,
        HeaderValue::from_str(&format!("Bearer {}", git_token))?,
    );

    //TODO: Should really cache this to prevent spamming attacks of this URL
    //maybe something like this:
    //https://docs.rs/http-cache-reqwest/latest/http_cache_reqwest/
    // initialize git api endpoint
    let git_api_url = "https://api.github.com/user/repos?sort=pushed&direction=desc";

    // return either the repo vector or a prorper error response string
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
