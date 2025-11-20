use serde::{Deserialize, Serialize};
#[derive(Debug, Serialize, Deserialize)]
pub struct FoodPreferences {
    #[serde(rename = "preferred_foods")]
    pub preferred_foods: Vec<String>,

    #[serde(rename = "allergies")]
    pub allergies: Vec<String>,
}
