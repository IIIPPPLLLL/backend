// controllers/schedule_controller.rs
use crate::{handler::auth_handler, middleware::authmiddleware::auth_middleware};
use crate::{handler::schedule_handler::create_schedule_handler, state::AppState};
use axum::{Router, routing::get, routing::post};

pub struct ScheduleController;

impl ScheduleController {
    pub fn routes() -> Router<AppState> {
        let protected_routes = Router::new()
            .route("/", post(create_schedule_handler))
            .layer(axum::middleware::from_fn(auth_middleware));

        Router::new().nest("/schedules", protected_routes)
    }
}
