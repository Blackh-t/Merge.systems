pub use crate::api::http_client::open_ai::client::ChatLog;
pub use crate::api::http_client::open_ai::converter::*;
use reqwest;
use std::error::Error;

/// Send request to OpenAI with the conversations historic.
/// # Parameter
/// - `chat_log`: list of tuplets ("Role", "Content")
async fn openai_send_request(
    chat_log: Vec<(String, String)>,
    key: String,
) -> Result<String, Box<dyn Error>> {
    // Convert ChatLog into Structure of conversations
    let msg = ChatLog::msg_convertion(chat_log, key).await.serialize()?;

    // Build request
    let url = "https://www.merge.systems/.netlify/functions/open_ai";
    let client = reqwest::Client::new();
    let response = client
        .post(url)
        .header("Content-Type", "application/json")
        .body(msg)
        .send()
        .await?;
    let body = response.text().await?;
    Ok(body)
}

/// Read response from HTTP-server
pub async fn openai_read_response(chat_log: Vec<(String, String)>, key: String) -> String {
    // OpenAi Chat Result Handler
    match openai_send_request(chat_log, key.clone()).await {
        Ok(msg) => format!("{}", msg),
        Err(e) => format!("Chat Serive: {}", e),
    }
}
