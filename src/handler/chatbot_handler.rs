// handlers/chatbot_handler.rs
use crate::{
    models::request::{ChatHistoryQuery, ChatRequest},
    services::chatbot_service::ChatbotService,
    state::AppState,
};
use axum::{
    Extension, Json,
    extract::{Query, State},
    http::StatusCode,
};
use log::info;
use mongodb::bson::oid::ObjectId;
use serde_json::{Value, json};
use std::sync::Arc;

pub async fn ask_chatbot_handler(
    State(app_state): State<AppState>,
    Extension(user_id): Extension<ObjectId>,
    Json(body): Json<ChatRequest>,
) -> Result<Json<Value>, StatusCode> {
    let chatbot_service = &app_state.chatbot_service;

    match chatbot_service.ask(user_id, body.message).await {
        Ok(response) => Ok(Json(json!({
            "success": true,
            "data": {
                "reply": response.reply,
                "actions": response.actions
            }
        }))),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

pub async fn get_chat_history_handler(
    State(app_state): State<AppState>, // <-- AppState here
    Extension(user_id): Extension<ObjectId>,
    Query(params): Query<ChatHistoryQuery>,
) -> Result<Json<Value>, StatusCode> {
    let limit = params.limit.unwrap_or(20);

    let chatbot_service = &app_state.chatbot_service;

    match chatbot_service
        .get_recent_conversations(user_id, limit)
        .await
    {
        Ok(messages) => Ok(Json(json!({
            "success": true,
            "data": {
                "messages": messages,
                "count": messages.len()
            }
        }))),
        Err(e) => {
            log::error!("Failed to get chat history: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

pub async fn clear_chat_history_handler(
    State(app_state): State<AppState>,
    Extension(user_id): Extension<ObjectId>,
) -> Result<Json<Value>, StatusCode> {
    let chatbot_service = &app_state.chatbot_service;

    match chatbot_service.clear_chat_history(user_id).await {
        Ok(_) => Ok(Json(json!({
            "success": true,
            "message": "Chat history cleared successfully"
        }))),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}
