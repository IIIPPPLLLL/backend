use crate::models::{loginrequest::LoginRequest, user::User};
use bcrypt::{DEFAULT_COST, hash, verify};
use mongodb::{
    Collection,
    bson::{Document, doc, oid::ObjectId}, // Import Document
};

#[derive(Clone)]
pub struct UserService {
    pub collection: Collection<Document>,
}

impl UserService {
    pub fn new_user_service(db: &mongodb::Database) -> Self {
        let collection = db.collection::<Document>("User"); // Ubah ke Document
        Self { collection }
    }

    pub async fn register_user(&self, user: User) -> mongodb::error::Result<()> {
        let hashed_password = hash(&user.password, DEFAULT_COST)
            .map_err(|e| mongodb::error::Error::custom(e.to_string()))?;

        let document = doc! {
            "username": &user.username,
            "email": &user.email,
            "password": hashed_password,
            "health_profile": mongodb::bson::to_bson(&user.health_profile)
                .map_err(|e| mongodb::error::Error::custom(e.to_string()))?,
            "food_preferences": mongodb::bson::to_bson(&user.food_preferences)
                .map_err(|e| mongodb::error::Error::custom(e.to_string()))?,
        };

        println!("Inserting document: {:?}", document);

        self.collection.insert_one(document).await?;
        Ok(())
    }

    pub async fn login_user(
        &self,
        name: String,
        password: String,
    ) -> mongodb::error::Result<Option<User>> {
        let filter = doc! {
            "$or": [
                { "username": &name },
                { "email": &name }
            ]
        };

        match self.collection.find_one(filter).await? {
            Some(doc) => match mongodb::bson::from_document::<User>(doc) {
                Ok(user) => match verify(&password, &user.password) {
                    Ok(true) => Ok(Some(user)),
                    Ok(false) => Ok(None),
                    Err(e) => Err(mongodb::error::Error::custom(e.to_string())),
                },
                Err(e) => {
                    eprintln!("Deserialization error: {:?}", e);
                    Err(mongodb::error::Error::from(e))
                }
            },
            None => Ok(None),
        }
    }

    pub async fn get_user_by_id(&self, id: ObjectId) -> mongodb::error::Result<Option<User>> {
        let filter = doc! { "_id": id };
        match self.collection.find_one(filter).await? {
            Some(doc) => mongodb::bson::from_document::<User>(doc)
                .map(Some)
                .map_err(|e| mongodb::error::Error::from(e)),
            None => Ok(None),
        }
    }
}
