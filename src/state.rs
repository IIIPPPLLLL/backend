use crate::services::user_services::UserService;
use mongodb::Database;

#[derive(Clone)]
pub struct AppState {
    pub db: Database,
    pub user_service: UserService,
}

impl AppState {
    pub fn new(db: Database) -> Self {
        let user_service = UserService::new_user_service(&db);

        Self { db, user_service }
    }
}
