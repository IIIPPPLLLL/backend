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
    // 1. Get user dengan full data
    let user = state
        .user_service
        .get_user_by_id(user_id)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .ok_or(StatusCode::NOT_FOUND)?;

    let recommendations = match user.food_preferences.recommendations {
        // Jika sudah ada recommendations, pakai yang ada
        Some(ref recs) if !recs.is_empty() => recs.clone(),
        // Jika belum ada, generate baru
        _ => state
            .utils_service
            .generate_recommend(user_id, &user.food_preferences)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?,
    };

    Ok(JsonResponse(serde_json::json!({
        "status": "success",
        "user_id": user_id.to_string(),
        "recommendations": recommendations,
        "count": recommendations.len()
    })))
}
