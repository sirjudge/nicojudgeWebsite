#[cfg(feature = "server")]
use crate::database::create_connection;
use dioxus::logger::tracing::{error, info};
use dioxus::{html::input, prelude::*};
#[cfg(feature = "server")]
use sqlx::{FromRow, Row};

#[cfg(feature = "server")]
use chrono::{DateTime, Utc};

#[component]
pub fn MaintenanceSettings() -> Element {
    let mut maintenance_box = use_signal(|| get_mode().await);
    rsx! {
        div {
            class: "maintenance-mode",
            h1 { "Maintenance Mode" }
            p { "The site is currently undergoing maintenance. Please check back later." }
            // You can add more details or a contact link here
            form {
                onsubmit:  move |_| {
                    spawn(async move {
                        if *maintenance_box.read() {
                            info!("enbabling maintenance_mode");
                            match save_mode(true).await {
                                Ok(_) => { info!("Enabled maintenance_mode");},
                                Err(e) => { error!("error ocured during enabling maintenance_mode:{}", e);}
                            };
                        }
                        else {
                            match save_mode(false).await {
                                Ok(_) => { info!("disabled maintenance_mode");},
                                Err(e) => { error!("error ocured during disabling maintenance_mode:{}", e);}
                            };
                        }
                    });
                },
                // Check box to turn site on and off of maintenance mode
                input {
                    r#type: "checkbox",
                    name: "maintenance_mode",
                    id: "maintenance_mode",
                    oninput: move |input_event| {
                        info!("maintenance_mode checkbox:{}",input_event.value());
                        maintenance_box.set(input_event.value() == "true");
                    }
                }
                button {
                    r#type: "submit",
                    "update maintenance_mode"
                }
            }
        }
    }
}

#[derive(Debug, Clone, Default)]
#[cfg_attr(feature = "server", derive(FromRow))]
pub struct WebFlags {
    pub maintenance_mode: bool,
    pub updated_date: chrono::DateTime<chrono::Utc>,
}

#[server]
async fn get_mode() -> Result<bool, ServerFnError> {
    match create_connection().await {
        Ok(mut conn) => {
            let result = sqlx::query_as::<_, WebFlags>(
                "select maintenance_mode
                from web_flags
                order by updated_Date desc
                limit 1",
            )
            .fetch_optional(&mut conn)
            .await;

            match result {
                Ok(query_result) => {
                    info!("Extracted query_results:{:?}", query_result);
                    Ok(query_result.unwrap().maintenance_mode)
                }

                Err(e) => {
                    let error_message = format!("error selecting maintenance mode:{e}");
                    return Err(ServerFnError::new(error_message));
                }
            }
        }
        Err(e) => {
            return Err(ServerFnError::new(format!(
                "database connection error: {e}"
            )));
        }
    }
}

#[server]
async fn save_mode(enabled: bool) -> Result<(), ServerFnError> {
    match create_connection().await {
        Ok(mut conn) => {
            info!("inserting new value for maintenance_mode:{}", enabled);
            let updated_date = Utc::now();
            let result = sqlx::query!(
                "INSERT INTO web_flags (maintenance_mode,updated_date) VALUES (?1,?2)",
                enabled,
                updated_date
            )
            .execute(&mut conn)
            .await;

            match result {
                Ok(query_result) => {
                    let inserted_id = query_result.last_insert_rowid() as i32;
                    return Ok(());
                }
                Err(e) => {
                    return Err(ServerFnError::new(format!(
                        "Error occurred during blog insert: {e}"
                    )));
                }
            }
        }
        Err(e) => {
            return Err(ServerFnError::new(format!(
                "database connection error: {e}"
            )));
        }
    }

    Ok(())
}
