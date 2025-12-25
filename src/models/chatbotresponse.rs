// models/chatbot_response.rs
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ChatbotResponse {
    pub reply: String,
    pub actions: Vec<ChatbotAction>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "action_type", content = "data")]
pub enum ChatbotAction {
    #[serde(rename = "create_notification")]
    CreateNotification {
        title: String,
        message: Option<String>,
        icon: Option<String>,
    },

    #[serde(rename = "recommend_meal")]
    RecommendMeal {
        meal_names: Vec<String>,
        reason: Option<String>,
    },

    #[serde(rename = "create_eat_schedule")]
    CreateEatSchedule {
        date: String,
        meal_time: String,
        meal_name: String,
        notes: Option<String>,
    },

    #[serde(rename = "create_shopping_schedule")]
    CreateShoppingSchedule {
        name: String,
        items: Vec<ScheduleItem>,
    },

    #[serde(other)]
    Unknown,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ScheduleItem {
    pub name: String,
    pub quantity: f64,
    pub unit: String,
    pub category: String,
}
