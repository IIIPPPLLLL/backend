use serde_json::json;

#[derive(Clone)]

pub struct OpenAiClient {
    pub api_key: String,
}

impl OpenAiClient {
    pub fn new(api_key: String) -> Self {
        Self { api_key }
    }
    pub async fn chat(&self, messages: Vec<serde_json::Value>) -> Result<String, anyhow::Error> {
        let response = reqwest::Client::new()
            .post("https://api.openai.com/v1/chat/completions")
            .bearer_auth(&self.api_key)
            .json(&json!({
                "model": "gpt-4o-mini",
                "messages": messages
            }))
            .send()
            .await?
            .json::<serde_json::Value>()
            .await?;

        Ok(response["choices"][0]["message"]["content"]
            .as_str()
            .unwrap_or("")
            .to_string())
    }
}
