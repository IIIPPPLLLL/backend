use crate::handler::preferences_handler::{
    add_food_preferences_handler, remove_food_preferences_handler,
};
use crate::state::AppState;
use crate::{handler::auth_handler, middleware::authmiddleware::auth_middleware};

use axum::{Router, routing::post};
#[derive(Clone)]
pub struct UserController;

impl UserController {
    pub fn routes() -> Router<AppState> {
        let auth_routes = Router::new()
            .route("/preferences/food", post(add_food_preferences_handler))
            .route("/preferences/remove", post(remove_food_preferences_handler))
            .layer(axum::middleware::from_fn(auth_middleware));

        Router::new()
            .route("/auth/register", post(auth_handler::register_user))
            .route("/auth/login", post(auth_handler::login))
            .merge(auth_routes)
    }
}
