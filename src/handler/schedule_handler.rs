use crate::{
    models::response::{
        CreateScheduleRequest, CreateShoppingItem, UpdateScheduleInfoRequest,
        UpdateShoppingItemRequest,
    },
    state::AppState,
};
use axum::{
    Extension,
    extract::{Json, Path, State},
    http::StatusCode,
    response::IntoResponse,
};
use mongodb::bson::oid::ObjectId;
use serde_json::json;

// 1. Create Schedule Handler
pub async fn create_schedule_handler(
    Extension(user_id): Extension<ObjectId>,
    State(state): State<AppState>,
    Json(payload): Json<CreateScheduleRequest>,
) -> impl IntoResponse {
    println!(
        "📥 Create shopping schedule request from user: {:?}",
        user_id
    );
    println!("📥 Schedule name: {}", payload.name);
    println!("📥 Number of items: {}", payload.items.len());

    if payload.name.trim().is_empty() {
        return (
            StatusCode::BAD_REQUEST,
            Json(json!({
                "status": "error",
                "message": "Schedule name is required"
            })),
        );
    }

    if payload.items.is_empty() {
        return (
            StatusCode::BAD_REQUEST,
            Json(json!({
                "status": "error",
                "message": "At least one shopping item is required"
            })),
        );
    }

    for (index, item) in payload.items.iter().enumerate() {
        if item.name.trim().is_empty() {
            return (
                StatusCode::BAD_REQUEST,
                Json(json!({
                    "status": "error",
                    "message": format!("Item {}: Name is required", index + 1)
                })),
            );
        }
        if item.quantity <= 0.0 {
            return (
                StatusCode::BAD_REQUEST,
                Json(json!({
                    "status": "error",
                    "message": format!("Item {}: Quantity must be greater than 0", index + 1)
                })),
            );
        }
    }

    match state
        .schedule_service
        .create_schedule(user_id, payload)
        .await
    {
        Ok(schedule) => {
            let shopping_responses: Vec<_> = schedule
                .shopping_list
                .iter()
                .enumerate()
                .map(|(index, item)| {
                    json!({
                        "index": index,
                        "id": Some(ObjectId::new()),
                        "name": item.name,
                        "quantity": item.quantity,
                        "unit": item.unit,
                        "category": item.category,
                        "purchased": item.purchased,
                        "notes": item.notes
                    })
                })
                .collect();

            (
                StatusCode::CREATED,
                Json(json!({
                    "status": "success",
                    "message": "Shopping schedule created successfully",
                    "data": {
                        "id": schedule.id.unwrap().to_string(),
                        "user_id": schedule.user_id.to_string(),
                        "name": schedule.name,
                        "description": schedule.description,
                        "shopping_list": shopping_responses,
                        "shopping_item_count": schedule.shopping_list.len(),
                        "created_at": schedule.created_at.to_rfc3339()
                    }
                })),
            )
        }
        Err(e) => {
            eprintln!("❌ Error creating shopping schedule: {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({
                    "status": "error",
                    "message": "Failed to create shopping schedule"
                })),
            )
        }
    }
}

pub async fn get_user_schedules_handler(
    Extension(user_id): Extension<ObjectId>,
    State(state): State<AppState>,
) -> impl IntoResponse {
    println!("📋 Getting all schedules for user: {:?}", user_id);

    match state.schedule_service.get_user_schedules(user_id).await {
        Ok(schedules) => {
            let response: Vec<_> = schedules
                .into_iter()
                .map(|schedule| {
                    json!({
                        "id": schedule.id.unwrap().to_string(),
                        "user_id": schedule.user_id.to_string(),
                        "name": schedule.name,
                        "description": schedule.description,
                        "shopping_item_count": schedule.shopping_list.len(),
                        "purchased_count": schedule.shopping_list.iter().filter(|item| item.purchased).count(),
                        "created_at": schedule.created_at.to_rfc3339()
                    })
                })
                .collect();

            (
                StatusCode::OK,
                Json(json!({
                    "status": "success",
                    "message": format!("Found {} schedules", response.len()),
                    "data": response
                })),
            )
        }
        Err(e) => {
            eprintln!("❌ Error fetching schedules: {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({
                    "status": "error",
                    "message": "Failed to fetch schedules"
                })),
            )
        }
    }
}

pub async fn get_schedule_by_id_handler(
    Extension(user_id): Extension<ObjectId>,
    State(state): State<AppState>,
    Path(schedule_id): Path<String>,
) -> impl IntoResponse {
    println!(
        "🔍 Getting schedule: {} for user: {:?}",
        schedule_id, user_id
    );

    let object_id = match ObjectId::parse_str(&schedule_id) {
        Ok(id) => id,
        Err(_) => {
            return (
                StatusCode::BAD_REQUEST,
                Json(json!({
                    "status": "error",
                    "message": "Invalid schedule ID format"
                })),
            );
        }
    };

    match state
        .schedule_service
        .get_schedule_by_id(object_id, user_id)
        .await
    {
        Ok(Some(schedule)) => {
            let shopping_responses: Vec<_> = schedule
                .shopping_list
                .iter()
                .enumerate()
                .map(|(index, item)| {
                    json!({
                        "index": index,
                        "id": item.id.as_ref().map(|id| id.to_string()),
                        "name": item.name,
                        "quantity": item.quantity,
                        "unit": item.unit,
                        "category": item.category,
                        "purchased": item.purchased,
                        "notes": item.notes
                    })
                })
                .collect();

            (
                StatusCode::OK,
                Json(json!({
                    "status": "success",
                    "data": {
                        "id": schedule.id.unwrap().to_string(),
                        "user_id": schedule.user_id.to_string(),
                        "name": schedule.name,
                        "description": schedule.description,
                        "shopping_list": shopping_responses,
                        "shopping_item_count": schedule.shopping_list.len(),
                        "purchased_count": schedule.shopping_list.iter().filter(|item| item.purchased).count(),
                        "created_at": schedule.created_at.to_rfc3339()
                    }
                })),
            )
        }
        Ok(None) => (
            StatusCode::NOT_FOUND,
            Json(json!({
                "status": "error",
                "message": "Schedule not found"
            })),
        ),
        Err(e) => {
            eprintln!("❌ Error fetching schedule: {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({
                    "status": "error",
                    "message": "Failed to fetch schedule"
                })),
            )
        }
    }
}

pub async fn delete_schedule_handler(
    Extension(user_id): Extension<ObjectId>,
    State(state): State<AppState>,
    Path(schedule_id): Path<String>,
) -> impl IntoResponse {
    println!(
        "🗑️ Deleting schedule: {} for user: {:?}",
        schedule_id, user_id
    );

    let object_id = match ObjectId::parse_str(&schedule_id) {
        Ok(id) => id,
        Err(_) => {
            return (
                StatusCode::BAD_REQUEST,
                Json(json!({
                    "status": "error",
                    "message": "Invalid schedule ID format"
                })),
            );
        }
    };

    match state
        .schedule_service
        .delete_schedule(object_id, user_id)
        .await
    {
        Ok(true) => (
            StatusCode::OK,
            Json(json!({
                "status": "success",
                "message": "Schedule deleted successfully"
            })),
        ),
        Ok(false) => (
            StatusCode::NOT_FOUND,
            Json(json!({
                "status": "error",
                "message": "Schedule not found"
            })),
        ),
        Err(e) => {
            eprintln!("❌ Error deleting schedule: {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({
                    "status": "error",
                    "message": "Failed to delete schedule"
                })),
            )
        }
    }
}

pub async fn update_shopping_item_handler(
    Extension(user_id): Extension<ObjectId>,
    State(state): State<AppState>,
    Path((schedule_id, item_index)): Path<(String, usize)>,
    Json(payload): Json<UpdateShoppingItemRequest>,
) -> impl IntoResponse {
    println!(
        "📝 Updating item {} in schedule {}",
        item_index, schedule_id
    );

    let schedule_object_id = match ObjectId::parse_str(&schedule_id) {
        Ok(id) => id,
        Err(_) => {
            return (
                StatusCode::BAD_REQUEST,
                Json(json!({
                    "status": "error",
                    "message": "Invalid schedule ID format"
                })),
            );
        }
    };

    let purchased = match payload.purchased {
        Some(p) => p,
        None => {
            return (
                StatusCode::BAD_REQUEST,
                Json(json!({
                    "status": "error",
                    "message": "Purchased status is required"
                })),
            );
        }
    };

    match state
        .schedule_service
        .update_shopping_item(schedule_object_id, item_index, user_id, purchased)
        .await
    {
        Ok(true) => (
            StatusCode::OK,
            Json(json!({
                "status": "success",
                "message": format!("Item {} updated successfully", item_index),
                "data": {
                    "item_index": item_index,
                    "purchased": purchased
                }
            })),
        ),
        Ok(false) => (
            StatusCode::NOT_FOUND,
            Json(json!({
                "status": "error",
                "message": "Schedule or item not found"
            })),
        ),
        Err(e) => {
            eprintln!("❌ Error updating item: {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({
                    "status": "error",
                    "message": "Failed to update item"
                })),
            )
        }
    }
}

pub async fn add_item_to_schedule_handler(
    Extension(user_id): Extension<ObjectId>,
    State(state): State<AppState>,
    Path(schedule_id): Path<String>,
    Json(payload): Json<CreateShoppingItem>,
) -> impl IntoResponse {
    println!("➕ Adding item to schedule {}", schedule_id);

    if payload.name.trim().is_empty() {
        return (
            StatusCode::BAD_REQUEST,
            Json(json!({
                "status": "error",
                "message": "Item name is required"
            })),
        );
    }
    if payload.quantity <= 0.0 {
        return (
            StatusCode::BAD_REQUEST,
            Json(json!({
                "status": "error",
                "message": "Quantity must be greater than 0"
            })),
        );
    }

    let schedule_object_id = match ObjectId::parse_str(&schedule_id) {
        Ok(id) => id,
        Err(_) => {
            return (
                StatusCode::BAD_REQUEST,
                Json(json!({
                    "status": "error",
                    "message": "Invalid schedule ID format"
                })),
            );
        }
    };

    match state
        .schedule_service
        .add_item_to_schedule(schedule_object_id, user_id, payload)
        .await
    {
        Ok(true) => (
            StatusCode::OK,
            Json(json!({
                "status": "success",
                "message": "Item added to schedule successfully"
            })),
        ),
        Ok(false) => (
            StatusCode::NOT_FOUND,
            Json(json!({
                "status": "error",
                "message": "Schedule not found"
            })),
        ),
        Err(e) => {
            eprintln!("❌ Error adding item: {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({
                    "status": "error",
                    "message": "Failed to add item"
                })),
            )
        }
    }
}

pub async fn remove_item_from_schedule_handler(
    Extension(user_id): Extension<ObjectId>,
    State(state): State<AppState>,
    Path((schedule_id, item_index)): Path<(String, usize)>,
) -> impl IntoResponse {
    println!(
        "➖ Removing item {} from schedule {}",
        item_index, schedule_id
    );

    let schedule_object_id = match ObjectId::parse_str(&schedule_id) {
        Ok(id) => id,
        Err(_) => {
            return (
                StatusCode::BAD_REQUEST,
                Json(json!({
                    "status": "error",
                    "message": "Invalid schedule ID format"
                })),
            );
        }
    };

    match state
        .schedule_service
        .remove_item_from_schedule(schedule_object_id, user_id, item_index)
        .await
    {
        Ok(true) => (
            StatusCode::OK,
            Json(json!({
                "status": "success",
                "message": format!("Item {} removed from schedule", item_index)
            })),
        ),
        Ok(false) => (
            StatusCode::NOT_FOUND,
            Json(json!({
                "status": "error",
                "message": "Schedule or item not found"
            })),
        ),
        Err(e) => {
            eprintln!("❌ Error removing item: {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({
                    "status": "error",
                    "message": "Failed to remove item"
                })),
            )
        }
    }
}

pub async fn update_schedule_info_handler(
    Extension(user_id): Extension<ObjectId>,
    State(state): State<AppState>,
    Path(schedule_id): Path<String>,
    Json(payload): Json<UpdateScheduleInfoRequest>,
) -> impl IntoResponse {
    println!("✏️ Updating schedule info: {}", schedule_id);

    let name_provided = payload.name.is_some();
    let description_provided = payload.description.is_some();

    if !name_provided && !description_provided {
        return (
            StatusCode::BAD_REQUEST,
            Json(json!({
                "status": "error",
                "message": "No fields to update"
            })),
        );
    }

    if let Some(ref name) = payload.name {
        if name.trim().is_empty() {
            return (
                StatusCode::BAD_REQUEST,
                Json(json!({
                    "status": "error",
                    "message": "Schedule name cannot be empty"
                })),
            );
        }
    }

    let schedule_object_id = match ObjectId::parse_str(&schedule_id) {
        Ok(id) => id,
        Err(_) => {
            return (
                StatusCode::BAD_REQUEST,
                Json(json!({
                    "status": "error",
                    "message": "Invalid schedule ID format"
                })),
            );
        }
    };

    match state
        .schedule_service
        .update_schedule_info(
            schedule_object_id,
            user_id,
            payload.name,
            payload.description,
        )
        .await
    {
        Ok(true) => {
            let mut updated_fields = Vec::new();

            if name_provided {
                updated_fields.push("name");
            }
            if description_provided {
                updated_fields.push("description");
            }

            (
                StatusCode::OK,
                Json(json!({
                    "status": "success",
                    "message": "Schedule updated successfully",
                    "updated_fields": updated_fields
                })),
            )
        }
        Ok(false) => (
            StatusCode::NOT_FOUND,
            Json(json!({
                "status": "error",
                "message": "Schedule not found"
            })),
        ),
        Err(e) => {
            eprintln!("❌ Error updating schedule: {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({
                    "status": "error",
                    "message": "Failed to update schedule"
                })),
            )
        }
    }
}
