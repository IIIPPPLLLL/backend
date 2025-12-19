use crate::models::{meals::Meal, shoppinglist::ShoppingItem};
use chrono;
use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};
#[derive(Debug, Serialize, Deserialize)]
pub struct Schedule {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,

    #[serde(rename = "user_id")]
    pub user_id: ObjectId,

    #[serde(rename = "name")]
    pub name: String,

    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    #[serde(rename = "shoppinglist")]
    pub shopping_list: Vec<ShoppingItem>,

    #[serde(rename = "created_at")]
    pub created_at: chrono::DateTime<chrono::Utc>,
}
