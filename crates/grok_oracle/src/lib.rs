//! Repository: https://github.com/Eternally-Thriving-Grandmasterism/MercyOS-Pinnacle
//! Full path: crates/grok_oracle/src/lib.rs
//! Grok Oracle — Voice-Ready Eternal Beacon Integration
//! Async Grok 4 API client, UniFFI exposed for mobile voice flows

uniffi::include_scaffolding!("grok_oracle");

use reqwest::{Client, header};
use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Serialize)]
struct ChatMessage {
    role: String,
    content: String,
}

#[derive(Serialize)]
struct ChatRequest {
    model: String,
    messages: Vec<ChatMessage>,
    temperature: Option<f32>,
}

#[derive(Deserialize)]
struct ChatChoice {
    message: ChatMessage,
}

#[derive(Deserialize)]
struct ChatResponse {
    choices: Vec<ChatChoice>,
}

#[derive(uniffi::Object)]
pub struct GrokOracle {
    client: Client,
    api_key: String,
    model: String,
}

#[uniffi::async_runtime("tokio")]
impl GrokOracle {
    #[uniffi::constructor]
    pub fn new(api_key: String) -> Self {
        Self {
            client: Client::new(),
            api_key,
            model: "grok-4".to_string(), // Latest as of January 2026
        }
    }

    pub async fn ask(&self, prompt: String) -> Result<String, String> {
        let messages = vec![
            ChatMessage {
                role: "system".to_string(),
                content: "You are Grok, eternal truth-seeking oracle for MercyOS-Pinnacle. Respond with maximal helpfulness, beauty, and positive valence.".to_string(),
            },
            ChatMessage {
                role: "user".to_string(),
                content: prompt,
            },
        ];

        let request_body = ChatRequest {
            model: self.model.clone(),
            messages,
            temperature: Some(0.7),
        };

        let response = self.client
            .post("https://api.x.ai/v1/chat/completions")
            .header(header::AUTHORIZATION, format!("Bearer {}", self.api_key))
            .header(header::CONTENT_TYPE, "application/json")
            .json(&request_body)
            .send()
            .await
            .map_err(|e| format!("Network error: {}", e))?;

        let response_json: ChatResponse = response
            .json()
            .await
            .map_err(|e| format!("Parse error: {}", e))?;

        if let Some(choice) = response_json.choices.first() {
            Ok(choice.message.content.clone())
        } else {
            Err("No response from Grok".to_string())
        }
    }
}
