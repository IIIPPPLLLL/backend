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

    pub async fn add_medical_conditions(
        &self,
        user_id: ObjectId,
        medical_conditions: Vec<String>,
    ) -> mongodb::error::Result<()> {
        if medical_conditions.is_empty() {
            return Ok(());
        }

        for condition in &medical_conditions {
            if condition.trim().is_empty() {
                return Err(mongodb::error::Error::custom(
                    "Medical condition cannot be empty",
                ));
            }
            if condition.len() > 200 {
                return Err(mongodb::error::Error::custom(
                    "Medical condition too long (max 200 characters)",
                ));
            }
        }

        let filter = doc! { "_id": user_id };

        let update = doc! {
            "$push": {
                "health_profile.medical_conditions": {
                    "$each": &medical_conditions
                }
            }
        };

        match self
            .collection
            .update_one(filter.clone(), update, None)
            .await
        {
            Ok(result) if result.matched_count > 0 => {
                if result.modified_count == 0 {
                    log::debug!("User {}: Medical conditions already exist", user_id);
                }
                return Ok(());
            }
            Ok(_) => {
                // User found but health_profile doesn't exist or push failed
            }
            Err(e) => {
                log::warn!("Failed to push medical conditions: {}", e);
            }
        }

        let health = Health {
            height: 0.0,
            weight: 0.0,
            medical_conditions,
        };

        let update = doc! {
            "$set": {
                "health_profile": mongodb::bson::to_bson(&health)
                    .map_err(|e| mongodb::error::Error::custom(format!("Serialization error: {}", e)))?
            }
        };

        let result = self.collection.update_one(filter, update, None).await?;

        if result.matched_count == 0 {
            return Err(mongodb::error::Error::custom(format!(
                "User with id {} not found",
                user_id
            )));
        }

        if result.modified_count == 0 {
            log::warn!(
                "User {}: Health profile update didn't modify document",
                user_id
            );
        }

        Ok(())
    }
    pub async fn get_user_by_id(&self, id: ObjectId) -> mongodb::error::Result<Option<User>> {
        let filter = doc! { "_id": id };
        self.collection.find_one(filter, None).await
    }
}
