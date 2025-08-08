use dioxus::prelude::*;

#[server]
pub async fn simple_test() -> Result<String, ServerFnError> {
    Ok("Hello from server!".to_string())
}

#[component]
pub fn TestComponent() -> Element {
    let mut message = use_signal(|| String::new());
    
    rsx! {
        div {
            h1 { "Server Function Test" }
            p { "Message: {message}" }
            button {
                onclick: move |_| {
                    spawn(async move {
                        match simple_test().await {
                            Ok(msg) => {
                                message.set(msg);
                            }
                            Err(e) => {
                                message.set(format!("Error: {}", e));
                            }
                        }
                    });
                },
                "Test Server Function"
            }
        }
    }
} 