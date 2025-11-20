use crate::handler::auth_handler;
use crate::state::AppState;

use axum::{Json, Router, routing::post};
#[derive(Clone)]
pub struct UserController;

impl UserController {
    pub fn routes() -> Router<AppState> {
        Router::new()
            .route("/auth/register", post(auth_handler::register_user))
            .route("/auth/login", post(auth_handler::login))
    }
}
