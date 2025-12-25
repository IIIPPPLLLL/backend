use mongodb::bson::{DateTime, oid::ObjectId};

pub struct ChatSession {
    pub id: Option<ObjectId>,
    pub user_id: ObjectId,
    pub title: String,
    pub created_at: DateTime,
}
