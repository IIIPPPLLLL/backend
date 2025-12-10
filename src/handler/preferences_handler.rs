use crate::models::foodpreferences::FoodPreferences;
use crate::state::AppState;
use axum::extract::{Json, State};
use axum::http::StatusCode;
use axum::response::Json as JsonResponse;

use axum::extract::Extension;
use mongodb::bson::oid::ObjectId;

pub async fn add_food_preferences_handler(
    State(state): State<AppState>,
    Extension(user_id): Extension<ObjectId>,
    Json(food_preferences): Json<FoodPreferences>,
) -> Result<JsonResponse<serde_json::Value>, StatusCode> {
    state
        .utils_service
        .add_food_preferences(user_id, food_preferences)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(JsonResponse(serde_json::json!({
        "status": "success",
        "message": "Food preferences added successfully"
    })))
}

pub async fn remove_food_preferences_handler(
    State(state): State<AppState>,
    Extension(user_id): Extension<ObjectId>,
) -> Result<JsonResponse<serde_json::Value>, StatusCode> {
    state
        .utils_service
        .remove_preferences(user_id)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(JsonResponse(serde_json::json!({
        "status": "success",
        "message": "Food preferences removed successfully"
    })))
}

pub async fn get_recommendations_handler(
    State(state): State<AppState>,
    Extension(user_id): Extension<ObjectId>,
) -> Result<JsonResponse<serde_json::Value>, StatusCode> {
    // 1. Get user preferences
    let preferences = state
        .utils_service
        .get_user_food_preferences(user_id)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .ok_or(StatusCode::NOT_FOUND)?;

    // 2. Generate recommendations
    let recommendations = state
        .utils_service
        .generate_recommend(user_id, &preferences)
        .await;
    Ok(JsonResponse(serde_json::json!({
        "status": "success",
        "recommendations": recommendations
    })))
}
