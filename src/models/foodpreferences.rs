use serde::{Deserialize, Serialize};

use crate::models::meals::Meal;
#[derive(Debug, Serialize, Deserialize)]
pub struct FoodPreferences {
    #[serde(rename = "preferred_foods")]
    pub preferred_foods: Vec<String>,

    #[serde(rename = "allergies")]
    pub allergies: Vec<String>,

    #[serde(rename = "recommendations")]
    pub recommendations: Option<Vec<Meal>>,
}
