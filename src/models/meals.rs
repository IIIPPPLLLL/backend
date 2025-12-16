// models/meals.rs
use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Meal {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,

    #[serde(rename = "name")]
    pub name: String,

    #[serde(rename = "ingredients")]
    pub ingredients: Vec<String>,

    #[serde(rename = "category")]
    pub category: String,

    #[serde(rename = "calories")]
    pub calories: Option<i32>,

    #[serde(rename = "image_url", skip_serializing_if = "Option::is_none")]
    pub image_url: Option<String>,

    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}
