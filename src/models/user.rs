use crate::models::foodpreferences::FoodPreferences;
use crate::models::health::Health;
use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};
#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    #[serde(rename = "_id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,

    #[serde(rename = "username")]
    pub username: String,

    #[serde(rename = "email")]
    pub email: String,

    #[serde(rename = "password")]
    pub password: String,

    #[serde(rename = "gender")]
    pub gender: Option<String>,

    #[serde(rename = "age")]
    pub age: Option<i32>,

    #[serde(rename = "physical_activity_level")]
    pub physical_activity_level: Option<String>,

    #[serde(rename = "goal")]
    pub goal: Option<String>,

    #[serde(rename = "health_profile")]
    pub health_profile: Option<Health>,

    #[serde(rename = "food_preferences")]
    pub food_preferences: Option<FoodPreferences>,
}
