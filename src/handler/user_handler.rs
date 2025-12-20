use crate::state::AppState;
use axum::response::{IntoResponse, Json as JsonResponse};
use axum::{Extension, Json, extract::State};
use hyper::StatusCode;
use mongodb::bson::oid::ObjectId;
use serde_json::json;

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

pub async fn get_current_user_handler(
    Extension(user_id): Extension<ObjectId>,
    State(state): State<AppState>,
) -> impl IntoResponse {
    println!("👤 Getting user profile for: {:?}", user_id);

    match state.user_service.get_user_by_id(user_id).await {
        Ok(Some(user)) => {
            let response = json!({
                "status": "success",
                "data": {
                    "id": user.id.unwrap().to_string(),
                    "username": user.username,
                    "email": user.email,
                    "age": user.age,
                    "gender": user.gender,
                    "physical_activity_level": user.physical_activity_level,
                    "goal": user.goal,
                    "health_profile": user.health_profile,
                    "food_preferences": user.food_preferences,
                }
            });

            (StatusCode::OK, Json(response))
        }
        Ok(None) => {
            println!("❌ User not found: {:?}", user_id);
            (
                StatusCode::NOT_FOUND,
                Json(json!({
                    "status": "error",
                    "message": "User not found"
                })),
            )
        }
        Err(e) => {
            eprintln!("❌ Error getting user: {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({
                    "status": "error",
                    "message": "Failed to get user profile"
                })),
            )
        }
    }
}

pub async fn get_user_health_profile_handler(
    State(state): State<AppState>,
    Extension(user_id): Extension<ObjectId>,
) -> Result<JsonResponse<serde_json::Value>, StatusCode> {
    let health_profile = state
        .user_service
        .get_health_profile(user_id)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    match health_profile {
        Some(profile) => Ok(JsonResponse(json!({
            "status": "success",
            "data": profile
        }))),
        None => Err(StatusCode::NOT_FOUND),
    }
}
pub async fn add_user_health_profile_handler(
    State(state): State<AppState>,
    Extension(user_id): Extension<ObjectId>,
    Json(health): Json<crate::models::health::Health>,
) -> Result<JsonResponse<serde_json::Value>, StatusCode> {
    state
        .user_service
        .add_health_profile(user_id, health)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(JsonResponse(json!({
        "status": "success",
        "message": "Health profile added successfully"
    })))
}
pub async fn delete_medical_conditions_handler(
    State(state): State<AppState>,
    Extension(user_id): Extension<ObjectId>,
) -> Result<JsonResponse<serde_json::Value>, StatusCode> {
    state
        .user_service
        .delete_medical_conditions(user_id)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(JsonResponse(json!({
        "status": "success",
        "message": "Medical conditions deleted successfully"
    })))
}
pub async fn update_medical_conditions_handler(
    State(state): State<AppState>,
    Extension(user_id): Extension<ObjectId>,
    Json(medical_conditions): Json<Vec<String>>,
) -> Result<JsonResponse<serde_json::Value>, StatusCode> {
    state
        .user_service
        .update_medical_conditions(user_id, medical_conditions)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(JsonResponse(json!({
        "status": "success",
        "message": "Medical conditions updated successfully"
    })))
}
