use leptos::*;


#[component]
pub fn Repos() -> impl IntoView {
    let repos = get_repos();
    view! {
        <div id="repos">
            <h1>"These are my repos!"</h1>
            <p>
                These are my repos 
            </p> 
            <ul>
                {match repos {
                    Ok(repos) => {
                        ReposLoaded(repos)
                    }
                    Err(_) => {
                        reposNotLoaded()
                    }
                }}
            </ul>
        </div>
    }
}

fn get_repos() -> impl Future<Item = Vec<repo>, Error = ()> {
    let url = "https://api.github.com/users/nicojudge/repos";
    let request = Request::new(url);
    let auth_token = format!("bearer {}", env!("GITHUB_AUTH_TOKEN"));
    request.headers().set("Authorization", auth_token);
    let future = fetch(request)
        .and_then(|response| response.json::<Vec<repo>>())
        .map_err(|_| ());
    future
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
