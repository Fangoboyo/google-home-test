use std::error::Error;

use homeassistant::HomeAssistantAPI;
use serde::Serialize;
use serde::Serializer;

#[derive(Serialize)]
pub struct ServiceData {
    command: String,
}

pub async fn turn_off_lights() -> Result<(), Box<dyn Error>> {
    let api = HomeAssistantAPI::new(
        "http://your-home-assistant-url".parse().unwrap(),
        "client-id".parse().unwrap(),
    );


    let data = ServiceData {
        command: "关掉次卧大灯".parse().unwrap()
    };
    // Acquire a read lock to call get_rest_client (a read operation)
    let read_guard = api.read().unwrap();
    let rest_client = read_guard.get_rest_client().await;

    // If service_call is a write operation, you would need to drop the read lock and acquire a write lock
    // drop(read_guard);
    // let mut write_guard = api.write().unwrap();
    // write_guard.get_rest_client().service_call("google_assistant_sdk.send_text_command", &service_data).await?;

    // If service_call is a read operation, you can just call it with the read lock
    rest_client.service_call::<ServiceData>(
        "http://your-home-assistant-url".parse().unwrap(),
        "google_assistant_sdk.send_text_command".parse().unwrap(),
        Some(data),
    ).await?;
    Ok(())
}
