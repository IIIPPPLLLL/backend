use crate::models::meals::Meal;
use crate::state::AppState;
use axum::extract::Json;
use axum::extract::Path;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::response::Json as JsonResponse;
use serde_json::json;

#[derive(Debug, serde::Serialize)]
pub struct MealResponse {
    pub id: String,
    pub name: String,
    pub ingredients: Vec<String>,
    pub category: String,
    pub calories: Option<i32>,
    pub image_url: Option<String>,
}

pub async fn get_all_meals_handler(State(state): State<AppState>) -> impl IntoResponse {
    match state.utils_service.get_all_meals().await {
        Ok(meals) => {
            let response_meals: Vec<MealResponse> = meals
                .into_iter()
                .map(|meal| MealResponse {
                    id: meal.id.unwrap().to_string(),
                    name: meal.name,
                    ingredients: meal.ingredients,
                    category: meal.category,
                    calories: meal.calories,
                    image_url: meal.image_url,
                })
                .collect();

            (
                StatusCode::OK,
                Json(json!({
                    "message": "Successfully retrieved meals",
                    "data": response_meals,
                    "count": response_meals.len()
                })),
            )
        }
        Err(e) => {
            eprintln!("Error getting meals: {:?}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({
                    "message": "Failed to retrieve meals",
                    "data": null,
                    "error": e.to_string()
                })),
            )
        }
    }
}

pub async fn add_meal_handler(
    State(state): State<AppState>,
    Json(meal): Json<Meal>,
) -> Result<JsonResponse<serde_json::Value>, StatusCode> {
    println!("📦 [HANDLER] Add meal request received");
    println!("📦 [HANDLER] Meal data: {:?}", meal);

    let meal_id = state.utils_service.add_meal(meal).await.map_err(|e| {
        eprintln!("❌ [HANDLER] Error adding meal: {:?}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    println!("✅ [HANDLER] Meal added with ID: {:?}", meal_id);

    Ok(JsonResponse(json!({
        "status": "success",
        "message": "Meal added successfully",
        "meal_id": meal_id.to_string()
    })))
}

pub async fn get_meal_by_id_handler(
    State(state): State<AppState>,
    Path(meal_id): Path<String>,
) -> Result<JsonResponse<serde_json::Value>, StatusCode> {
    let meal_object_id =
        mongodb::bson::oid::ObjectId::parse_str(&meal_id).map_err(|_| StatusCode::BAD_REQUEST)?;

    match state.utils_service.get_meal_by_id(meal_object_id).await {
        Ok(Some(meal)) => {
            let response_meal = MealResponse {
                id: meal.id.map(|id| id.to_string()).unwrap_or_default(),
                name: meal.name,
                ingredients: meal.ingredients,
                category: meal.category,
                calories: meal.calories,
                image_url: meal.image_url,
            };

            Ok(JsonResponse(json!({
                "status": "success",
                "message": "Meal retrieved successfully",
                "data": response_meal
            })))
        }
        Ok(None) => Err(StatusCode::NOT_FOUND),
        Err(e) => {
            eprintln!("Error getting meal by ID: {:?}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}
