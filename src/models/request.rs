use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct UpdateGenderRequest {
    pub gender: String,
}
#[derive(Debug, Deserialize)]
pub struct UpdateWeightRequest {
    pub weight: f64,
    pub unit: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateHeightRequest {
    pub height: f64,
    pub unit: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateAgeRequest {
    pub age: u32,
    #[serde(default)]
    pub date_of_birth: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateHealthProfileRequest {
    pub height: Option<f64>,
    pub weight: Option<f64>,
    pub medical_conditions: Option<Vec<String>>,
}
