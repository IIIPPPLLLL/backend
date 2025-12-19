use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ShoppingItem {
    #[serde(rename = "_id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,

    #[serde(rename = "user_id")]
    pub user_id: ObjectId,

    #[serde(rename = "name")]
    pub name: String,

    #[serde(rename = "quantity")]
    pub quantity: f64,

    #[serde(rename = "unit")]
    pub unit: String,

    #[serde(rename = "category")]
    pub category: String,

    #[serde(rename = "purchased")]
    pub purchased: bool,

    #[serde(rename = "notes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
}
