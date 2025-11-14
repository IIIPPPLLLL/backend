use crate::services::user_service::UserService;
use mongodb::Database;

#[derive(Clone)]
pub struct AppState {
    pub db: Database,
    pub user_service: UserService,
}

impl AppState {
    pub fn new(db: Database) -> Self {
        Self {
            user_service: UserService { db: db.clone() },
            db,
        }
    }
}
