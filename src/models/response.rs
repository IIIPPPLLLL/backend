// models/schedule_request.rs
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct ScheduleResponse {
    pub id: String,
    pub user_id: String,
    pub meals: Vec<MealResponse>,
    pub shopping_list: Vec<ShoppingItemResponse>,
    pub meal_count: usize,
    pub shopping_item_count: usize,
}

#[derive(Debug, Serialize)]
pub struct MealResponse {
    pub id: String,
    pub name: String,
    pub ingredients: Vec<String>,
    pub category: String,
    pub calories: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_url: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct ShoppingItemResponse {
    pub id: Option<String>,
    pub name: String,
    pub quantity: f64,
    pub unit: String,
    pub category: String,
    pub purchased: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
}
#[derive(Debug, Deserialize)]
pub struct CreateScheduleRequest {
    pub name: String,
    pub description: Option<String>,
    pub items: Vec<CreateShoppingItem>, // User input items langsung
}

#[derive(Debug, Deserialize)]
pub struct CreateShoppingItem {
    pub name: String,
    pub quantity: f64,
    pub unit: String,
    pub category: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
}

#[derive(Debug, serde::Deserialize)]
pub struct UpdateScheduleInfoRequest {
    pub name: Option<String>,
    pub description: Option<String>,
}

#[derive(Debug, serde::Deserialize)]
pub struct UpdateShoppingItemRequest {
    pub purchased: Option<bool>,
}

#[derive(Debug, Deserialize)]
pub struct AddMedicalConditionsRequest {
    #[serde(default)]
    pub medical_conditions: Vec<String>,
}
