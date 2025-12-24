use crate::handler::notification_handler::{
    create_notification_handler, get_notifications_handler, mark_notification_as_read_handler,
};
use crate::middleware::authmiddleware::auth_middleware;
use crate::state::AppState;

use axum::{
    Router,
    routing::{get, patch, post},
};

#[derive(Clone)]
pub struct NotificationController;

impl NotificationController {
    pub fn routes() -> Router<AppState> {
        let auth_routes = Router::new()
            // GET /notifications?limit=50
            .route("/", get(get_notifications_handler))
            // POST /notifications
            .route("/", post(create_notification_handler))
            // PATCH /notifications/{id}/read
            .route("/{id}/read", patch(mark_notification_as_read_handler))
            .layer(axum::middleware::from_fn(auth_middleware));

        Router::new().nest("/notifications", auth_routes)
    }
}
