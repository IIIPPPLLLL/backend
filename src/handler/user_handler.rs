use crate::models::request::{
    UpdateAgeRequest, UpdateGenderRequest, UpdateGoalRequest, UpdateHealthProfileRequest,
    UpdateHeightRequest, UpdatePhysicalActivityRequest, UpdateWeightRequest,
};
use crate::models::response::AddMedicalConditionsRequest;
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
pub async fn update_user_health_profile_handler(
    State(state): State<AppState>,
    Extension(user_id): Extension<ObjectId>,
    Json(payload): Json<UpdateHealthProfileRequest>,
) -> Result<JsonResponse<serde_json::Value>, StatusCode> {
    state
        .user_service
        .update_health_profile(user_id, payload)
        .await
        .map_err(|e| {
            eprintln!("Update health profile error: {:?}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    Ok(JsonResponse(json!({
        "status": "success",
        "message": "Health profile updated"
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
pub async fn add_medical_conditions_handler(
    State(state): State<AppState>,
    Extension(user_id): Extension<ObjectId>,
    Json(payload): Json<AddMedicalConditionsRequest>,
) -> Result<JsonResponse<serde_json::Value>, (StatusCode, String)> {
    if payload.medical_conditions.is_empty() {
        return Ok(JsonResponse(serde_json::json!({
            "status": "success",
            "message": "No medical conditions provided"
        })));
    }

    match state
        .user_service
        .add_medical_conditions(user_id, payload.medical_conditions)
        .await
    {
        Ok(_) => Ok(JsonResponse(serde_json::json!({
            "status": "success",
            "message": "Medical conditions added successfully",
            "user_id": user_id.to_string()
        }))),

        Err(e) => {
            let error_msg = e.to_string();

            if error_msg.contains("not found") {
                return Err((StatusCode::NOT_FOUND, "User not found".to_string()));
            }

            if error_msg.contains("cannot be empty")
                || error_msg.contains("too long")
                || error_msg.contains("Serialization error")
            {
                return Err((StatusCode::BAD_REQUEST, error_msg));
            }

            eprintln!("Internal server error: {}", error_msg);
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to add medical conditions".to_string(),
            ))
        }
    }
}

pub async fn update_user_age_handler(
    State(state): State<AppState>,
    Extension(user_id): Extension<ObjectId>,
    Json(payload): Json<UpdateAgeRequest>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    if payload.age < 1 || payload.age > 120 {
        return Err((
            StatusCode::BAD_REQUEST,
            "Age must be between 1 and 120 years".to_string(),
        ));
    }

    match state
        .user_service
        .update_user_age(user_id, payload.age as i32)
        .await
    {
        Ok(_) => Ok((
            StatusCode::OK,
            Json(json!({
                "status": "success",
                "message": "Age updated successfully",
                "user_id": user_id.to_string(),
                "age": payload.age,
                "age_group": match payload.age {
                    0..=12 => "child",
                    13..=19 => "teenager",
                    20..=39 => "young adult",
                    40..=59 => "middle aged",
                    _ => "senior",
                }
            })),
        )),
        Err(e) => {
            let error_msg = e.to_string();

            if error_msg.contains("not found") {
                return Err((StatusCode::NOT_FOUND, "User not found".to_string()));
            }

            eprintln!("Error updating age: {}", error_msg);
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to update age".to_string(),
            ))
        }
    }
}
pub async fn update_gender_handler(
    State(state): State<AppState>,
    Extension(user_id): Extension<ObjectId>,
    Json(payload): Json<UpdateGenderRequest>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let gender = payload.gender.trim();
    if gender.is_empty() {
        return Err((
            StatusCode::BAD_REQUEST,
            "Gender cannot be empty".to_string(),
        ));
    }

    match state
        .user_service
        .add_user_gender(user_id, gender.to_string())
        .await
    {
        Ok(_) => {
            let json_response = serde_json::json!({
                "status": "success",
                "message": "Gender updated successfully",
                "user_id": user_id.to_string(),
                "gender": gender
            });
            Ok((StatusCode::OK, Json(json_response)))
        }
        Err(e) => {
            let error_msg = e.to_string();

            if error_msg.contains("not found") {
                return Err((StatusCode::NOT_FOUND, "User not found".to_string()));
            }

            if error_msg.contains("Invalid gender") || error_msg.contains("cannot be empty") {
                return Err((StatusCode::BAD_REQUEST, error_msg));
            }

            eprintln!("Error updating gender: {}", error_msg);
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to update gender".to_string(),
            ))
        }
    }
}
pub async fn update_physical_activity_level_handler(
    State(state): State<AppState>,
    Extension(user_id): Extension<ObjectId>,
    Json(payload): Json<UpdatePhysicalActivityRequest>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let level = payload.physical_activity_level.trim().to_lowercase();

    let allowed = ["beginner", "intermediate", "advance"];
    if !allowed.contains(&level.as_str()) {
        return Err((
            StatusCode::BAD_REQUEST,
            "physical_activity_level must be one of: Beginner, Intermediate, Advance".to_string(),
        ));
    }

    state
        .user_service
        .update_physical_activity_level(user_id, level)
        .await
        .map_err(|e| {
            eprintln!("Update physical activity error: {:?}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to update activity level".to_string(),
            )
        })?;

    Ok((
        StatusCode::OK,
        Json(serde_json::json!({
            "status": "success",
            "message": "Physical activity level updated"
        })),
    ))
}
pub async fn update_goal_handler(
    State(state): State<AppState>,
    Extension(user_id): Extension<ObjectId>,
    Json(payload): Json<UpdateGoalRequest>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let goal = payload.goal.trim().to_lowercase();

    state
        .user_service
        .update_goal(user_id, goal)
        .await
        .map_err(|e| {
            eprintln!("Update goals error: {:?}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to update goals".to_string(),
            )
        })?;

    Ok((
        StatusCode::OK,
        Json(serde_json::json!({
            "status": "success",
            "message": "Goals updated"
        })),
    ))
}
