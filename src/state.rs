use crate::{
    models::user::User,
    services::{
        user_services::UserService,
        utils_service::{self, UtilsService},
    },
};
use mongodb::Database;

#[derive(Clone)]
pub struct AppState {
    pub db: Database,
    pub user_service: UserService,
    pub utils_service: UtilsService,
}

impl AppState {
    pub fn new(db: Database) -> Self {
        let user_collection = db.collection::<User>("users");
        let user_service = UserService::new(user_collection.clone());
        let utils_service = UtilsService::new(user_collection);
        Self {
            db,
            user_service,
            utils_service,
        }
    }
}
