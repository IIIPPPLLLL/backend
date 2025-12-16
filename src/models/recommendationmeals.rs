use serde::{Deserialize, Serialize};

use crate::models::meals::Meal;
#[derive(Debug, Serialize, Deserialize)]
pub struct RecommendationMeals {
    #[serde(rename = "recommendations_meals")]
    pub recommendations: Vec<Meal>,
}
