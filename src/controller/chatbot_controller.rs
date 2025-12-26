use crate::{
    handler::chatbot_handler::{
        ask_chatbot_handler, clear_chat_history_handler, get_chat_history_handler,
    },
    middleware::authmiddleware::auth_middleware,
    state::AppState,
};

use axum::{
    Router,
    extract::{Path, Query},
    middleware,
    routing::{delete, get, post},
};

#[derive(Clone)]
pub struct ChatbotController;

impl ChatbotController {
    pub fn routes() -> Router<AppState> {
        let auth_routes = Router::new()
            .route("/ask", post(ask_chatbot_handler))
            .route("/history", get(get_chat_history_handler))
            .route("/history/clear", delete(clear_chat_history_handler))
            .layer(middleware::from_fn(auth_middleware));

        Router::new().nest("/chatbot", auth_routes)
    }
}
