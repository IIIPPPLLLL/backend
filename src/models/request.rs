use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct UpdateGenderRequest {
    pub gender: String,
}
#[derive(Debug, Deserialize)]
pub struct UpdateWeightRequest {
    pub weight: f64,
    pub unit: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateHeightRequest {
    pub height: f64,
    pub unit: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateAgeRequest {
    pub age: u32,
    #[serde(default)]
    pub date_of_birth: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateHealthProfileRequest {
    pub height: Option<f64>,
    pub weight: Option<f64>,
    pub medical_conditions: Option<Vec<String>>,
}

#[derive(Deserialize)]
pub struct UpdatePhysicalActivityRequest {
    pub physical_activity_level: String,
}

#[derive(Deserialize)]
pub struct UpdateGoalRequest {
    pub goal: String,
}

#[derive(Debug, Deserialize)]
pub struct UpdateFoodPreferencesRequest {
    pub preferred_foods: Option<Vec<String>>,
    pub allergies: Option<Vec<String>>,
}

#[derive(Debug, Deserialize)]
pub struct CreateEatScheduleRequest {
    pub date: String,
    pub meal_time: String,
    pub meal_id: String,
    pub notes: Option<String>,
}

#[derive(Debug, serde::Deserialize)]
pub struct DateRangeRequest {
    pub start_date: String, // Format: YYYY-MM-DD
    pub end_date: String,   // Format: YYYY-MM-DD
}

#[derive(Debug, Deserialize)]
pub struct CreateNotificationRequest {
    pub title: String,
    pub message: Option<String>,
    pub icon: Option<String>,
}
