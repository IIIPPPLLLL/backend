use axum::{Router, routing::post};

use crate::{handler::auth_handler, state::AppState};

pub struct AuthController;

impl AuthController {
    pub fn routes() -> Router<AppState> {
        Router::new()
            .route("/auth/register", post(auth_handler::register_user))
            .route("/auth/login", post(auth_handler::login))
    }
}
