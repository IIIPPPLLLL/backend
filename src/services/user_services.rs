use crate::models::user::User;
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

    pub async fn get_user_by_id(&self, id: ObjectId) -> mongodb::error::Result<Option<User>> {
        let filter = doc! { "_id": id };
        self.collection.find_one(filter, None).await // Langsung return User
    }
}
