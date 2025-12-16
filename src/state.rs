use crate::{
    models::{meals::Meal, user::User},
    services::{user_services::UserService, utils_service::UtilsService},
};
use mongodb::Database;
use std::sync::Arc;

#[derive(Clone)]
pub struct AppState {
    pub db: Database,
    pub user_service: Arc<UserService>,
    pub utils_service: Arc<UtilsService>,
}
impl AppState {
    pub fn new(db: Database) -> Self {
        let user_collection = db.collection::<User>("users");
        let meals_collection = db.collection::<Meal>("meals");
        let user_service = Arc::new(UserService::new(user_collection.clone()));
        let utils_service = Arc::new(UtilsService::new(user_collection, meals_collection));
        Self {
            db,
            user_service,
            utils_service,
        }
    }
}
