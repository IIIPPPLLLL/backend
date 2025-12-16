use crate::models::meals::Meal;
use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]

pub struct Schedule {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    #[serde(rename = "meals")]
    pub meals: Meal,
    #[serde(rename = "shoppinglist")]
    pub shopping_list: Vec<String>,
}
