use axum::{
    Json,
    extract::{Extension, Path, Query, State},
    http::StatusCode,
};
use mongodb::bson::oid::ObjectId;
use serde::Deserialize;
use serde_json::json;

use crate::{models::request::CreateNotificationRequest, state::AppState};

#[derive(Deserialize)]
pub struct PaginationQuery {
    pub limit: Option<i64>,
}

pub async fn create_notification_handler(
    State(state): State<AppState>,
    Extension(user_id): Extension<ObjectId>,
    Json(payload): Json<CreateNotificationRequest>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    state
        .notification_service
        .create_notification(user_id, payload)
        .await
        .map_err(|e| {
            eprintln!("❌ Create notification error: {:?}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    Ok(Json(serde_json::json!({
        "status": "success",
        "message": "Notification created"
    })))
}

pub async fn get_notifications_handler(
    State(state): State<AppState>,
    Extension(user_id): Extension<ObjectId>,
    Query(query): Query<PaginationQuery>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    let limit = query.limit.unwrap_or(50);

    let notifications = state
        .notification_service
        .get_user_notifications(user_id, limit)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(json!({
        "status": "success",
        "data": notifications
    })))
}

pub async fn mark_notification_as_read_handler(
    State(state): State<AppState>,
    Extension(user_id): Extension<ObjectId>,
    Path(notification_id): Path<String>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    let notification_id =
        ObjectId::parse_str(&notification_id).map_err(|_| StatusCode::BAD_REQUEST)?;

    let updated = state
        .notification_service
        .mark_as_read(notification_id, user_id)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    if !updated {
        return Err(StatusCode::NOT_FOUND);
    }

    Ok(Json(json!({
        "status": "success",
        "message": "Notification marked as read"
    })))
}
