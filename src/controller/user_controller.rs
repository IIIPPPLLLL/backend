use crate::handler::meal_handler::{add_meal_handler, get_all_meals_handler};
use crate::handler::preferences_handler::{
    add_food_preferences_handler, get_recommendations_handler, remove_food_preferences_handler,
};
use crate::handler::user_handler::{
    add_user_health_profile_handler, delete_medical_conditions_handler, get_current_user_handler,
    get_user_health_profile_handler, get_user_preferences_handler,
    update_medical_conditions_handler,
};
use crate::state::AppState;
use crate::{handler::auth_handler, middleware::authmiddleware::auth_middleware};

use axum::routing::{delete, get, put};
use axum::{Router, routing::post};
#[derive(Clone)]
pub struct UserController;

impl UserController {
    pub fn routes() -> Router<AppState> {
        let auth_routes = Router::new()
            .route("/preferences/food", post(add_food_preferences_handler))
            .route("/user/preferences", get(get_user_preferences_handler))
            .route("/preferences/remove", post(remove_food_preferences_handler))
            .route(
                "/recommendations/generate",
                get(get_recommendations_handler),
            )
            .route("/meals/getAll", get(get_all_meals_handler))
            .route("/meals/add", post(add_meal_handler))
            .route("/profile", get(get_current_user_handler))
            .route("/health/profile", get(get_user_health_profile_handler))
            .route("/health/profile/add", post(add_user_health_profile_handler))
            .route(
                "/health/medical_conditions/delete",
                delete(delete_medical_conditions_handler),
            )
            .route(
                "/health/medical_conditions/update",
                put(update_medical_conditions_handler),
            )
            .layer(axum::middleware::from_fn(auth_middleware));

        Router::new()
            .route("/auth/register", post(auth_handler::register_user))
            .route("/auth/login", post(auth_handler::login))
            .merge(auth_routes)
    }
}
