use serde::{Deserialize, Serialize};
#[derive(Debug, Serialize, Deserialize)]
pub struct RecommendationMeals {
    #[serde(rename = "recommendations_meals")]
    pub recommendations: Vec<String>,
}
