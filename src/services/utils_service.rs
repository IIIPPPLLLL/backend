use mongodb::{
    Collection,
    bson::{Document, oid::ObjectId},
};

#[derive(Clone)]
pub struct UtilsService {
    pub collection: Collection<Document>,
}

impl UtilsService {
    pub async fn add_food_preferences(&self, userID: ObjectId) {}
}
