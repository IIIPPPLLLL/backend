use crate::models::{health::Health, user::User};
use bcrypt::{DEFAULT_COST, hash, verify};
use mongodb::{
    Collection,
    bson::{doc, oid::ObjectId},
};

#[derive(Clone)]
pub struct UserService {
    pub collection: Collection<User>,
}

impl UserService {
    pub fn new(collection: Collection<User>) -> Self {
        Self { collection }
    }

    pub async fn register_user(&self, mut user: User) -> mongodb::error::Result<()> {
        let hashed_password = hash(&user.password, DEFAULT_COST)
            .map_err(|e| mongodb::error::Error::custom(e.to_string()))?;

        user.password = hashed_password;

        self.collection.insert_one(user, None).await?;
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

        match self.collection.find_one(filter, None).await? {
            Some(user) => {
                // Verify password
                match verify(&password, &user.password) {
                    Ok(true) => Ok(Some(user)),
                    Ok(false) => Ok(None),
                    Err(e) => Err(mongodb::error::Error::custom(e.to_string())),
                }
            }
            None => Ok(None),
        }
    }

    pub async fn get_health_profile(
        &self,
        user_id: ObjectId,
    ) -> mongodb::error::Result<Option<Health>> {
        let filter = doc! { "_id": user_id };
        match self.collection.find_one(filter, None).await? {
            Some(user) => Ok(user.health_profile),
            None => Ok(None),
        }
    }

    pub async fn add_health_profile(
        &self,
        user_id: ObjectId,
        health: Health,
    ) -> mongodb::error::Result<()> {
        let filter = doc! { "_id": user_id };
        let update = doc! {
            "$set": {
                "health_profile": mongodb::bson::to_bson(&health)?
            }
        };

        self.collection.update_one(filter, update, None).await?;
        Ok(())
    }

    pub async fn delete_medical_conditions(&self, user_id: ObjectId) -> mongodb::error::Result<()> {
        let filter = doc! { "_id": user_id };
        let update = doc! {
            "$set": {
                "health_profile.medical_conditions": []
            }
        };

        self.collection.update_one(filter, update, None).await?;
        Ok(())
    }

    pub async fn update_medical_conditions(
        &self,
        user_id: ObjectId,
        medical_conditions: Vec<String>,
    ) -> mongodb::error::Result<()> {
        let filter = doc! { "_id": user_id };
        let update = doc! {
            "$set": {
                "health_profile.medical_conditions": medical_conditions
            }
        };

        self.collection.update_one(filter, update, None).await?;
        Ok(())
    }

    pub async fn get_user_by_id(&self, id: ObjectId) -> mongodb::error::Result<Option<User>> {
        let filter = doc! { "_id": id };
        self.collection.find_one(filter, None).await
    }
}
