use crate::state::AppState;
use axum::response::Json as JsonResponse;
use axum::{Extension, Json, extract::State};
use hyper::StatusCode;
use mongodb::bson::oid::ObjectId;

pub async fn get_user_preferences_handler(
    State(state): State<AppState>,
    Extension(user_id): Extension<ObjectId>,
) -> Result<JsonResponse<serde_json::Value>, StatusCode> {
    let user = state
        .user_service
        .get_user_by_id(user_id)
        .await
        .map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?
        .ok_or(axum::http::StatusCode::NOT_FOUND)?;

    let user_prefer = state
        .utils_service
        .get_user_food_preferences(user_id)
        .await
        .map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(Json(serde_json::json!({
        "food_preferences": user_prefer,
    })))
}
