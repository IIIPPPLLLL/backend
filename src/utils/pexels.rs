use reqwest::Client;
use serde_json::Value;
use std::env;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum PexelsError {
    #[error("API key not found")]
    ApiKeyMissing,

    #[error("Request failed: {0}")]
    RequestFailed(#[from] reqwest::Error),

    #[error("No photos found for query: {0}")]
    NoPhotosFound(String),

    #[error("Invalid response format")]
    InvalidResponse,
}

pub async fn get_meal_image(query: &str) -> Result<Option<String>, PexelsError> {
    let api_key = env::var("PEXELS_API_KEY").map_err(|_| PexelsError::ApiKeyMissing)?;

    let url = format!(
        "https://api.pexels.com/v1/search?query={}&per_page=1",
        query.replace(" ", "+") // Use + instead of %20 (more standard)
    );

    let client = Client::new();
    let res = client
        .get(&url)
        .header("Authorization", api_key)
        .timeout(std::time::Duration::from_secs(10)) // Add timeout
        .send()
        .await?;

    // Check HTTP status
    if !res.status().is_success() {
        let status = res.status();
        let body = res.text().await.unwrap_or_default();
        eprintln!("Pexels API error ({}): {}", status, body);
        return Ok(None); // Return None instead of failing
    }

    let json: Value = res.json().await?;

    // Check if photos array exists and is not empty
    let photos = json
        .get("photos")
        .and_then(|p| p.as_array())
        .ok_or(PexelsError::InvalidResponse)?;

    if photos.is_empty() {
        eprintln!("No photos found for query: {}", query);
        return Ok(None);
    }

    // Safely extract image URL
    let photo = &photos[0];
    let image_url = photo
        .get("src")
        .and_then(|src| src.get("large"))
        .and_then(|url| url.as_str())
        .map(|s| s.to_string())
        .ok_or(PexelsError::InvalidResponse)?;

    Ok(Some(image_url))
}
