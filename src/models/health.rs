use serde::{Deserialize, Serialize};
#[derive(Debug, Serialize, Deserialize)]
pub struct Health {
    #[serde(rename = "height")]
    pub height: f64,

    #[serde(rename = "weight")]
    pub weight: f64,

    #[serde(rename = "medical_conditions")]
    pub medical_conditions: Vec<String>,
}
