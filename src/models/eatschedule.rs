use mongodb::bson::{DateTime, oid::ObjectId};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct EatSchedule {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,

    #[serde(rename = "user_id")]
    pub user_id: ObjectId,

    #[serde(rename = "date")]
    pub date: String,

    #[serde(rename = "meal_time")]
    pub meal_time: String,

    #[serde(rename = "meal_id")]
    pub meal_id: ObjectId,

    #[serde(rename = "notes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,

    #[serde(rename = "created_at")]
    pub created_at: DateTime,
}
