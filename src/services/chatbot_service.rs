// chatbot_service.rs
use crate::{
    models::{
        chatbotresponse::{ChatbotAction, ChatbotResponse, ScheduleItem},
        chatmessage::ChatMessage,
        eatschedule::EatSchedule,
        request::CreateEatScheduleRequest,
        response::CreateScheduleRequest,
        user::User,
    },
    services::{
        notification_service::NotificationService, schedule_services::ScheduleService,
        user_services::UserService, utils_service::UtilsService,
    },
    utils::{self, llm::openai_client::OpenAiClient},
};
use futures_util::TryStreamExt;
use mongodb::{
    Collection,
    bson::{DateTime, doc, oid::ObjectId, to_bson},
};
use serde_json::{Value, json};
use std::sync::Arc;

pub struct ChatbotService {
    chat_collection: Collection<ChatMessage>,
    openai: OpenAiClient,
    user_service: Arc<UserService>,
    schedule_service: Arc<ScheduleService>,
    notification_service: Arc<NotificationService>,
    utils_service: Arc<UtilsService>,
}

impl ChatbotService {
    pub fn new(
        chat_collection: Collection<ChatMessage>,
        openai: OpenAiClient,
        user_service: Arc<UserService>,
        schedule_service: Arc<ScheduleService>,
        notification_service: Arc<NotificationService>,
        utils_service: Arc<UtilsService>,
    ) -> Self {
        Self {
            chat_collection,
            openai,
            user_service,
            schedule_service,
            notification_service,
            utils_service,
        }
    }

    /// Main entry point for chatbot interaction
    pub async fn ask(
        &self,
        user_id: ObjectId,
        user_message: String,
    ) -> Result<ChatbotResponse, Box<dyn std::error::Error>> {
        // 1. Get user profile
        let user = self
            .user_service
            .get_user_by_id(user_id)
            .await?
            .ok_or("User not found")?;

        // 2. Get today's schedules for context
        let today = chrono::Utc::now().date_naive();
        let schedules = self
            .schedule_service
            .get_schedules_by_date(user_id, today)
            .await
            .unwrap_or_default();

        // 3. Build comprehensive system prompt
        let system_prompt = self.build_system_prompt(&user, &schedules).await?;

        // 4. Get chat history (last 10 messages)
        let history = self.get_chat_history(user_id, 10).await?;

        // 5. Build messages for OpenAI
        let mut messages = vec![json!({ "role": "system", "content": system_prompt })];

        for msg in history {
            messages.push(json!({
                "role": msg.role,
                "content": msg.content
            }));
        }

        // 6. Add current user message
        messages.push(json!({
            "role": "user",
            "content": user_message
        }));

        // 7. Save user message to database
        self.save_message(user_id, "user", &user_message).await?;

        // 8. Call OpenAI with JSON response parsing
        let response_str = self
            .openai
            .chat(messages.clone())
            .await
            .map_err(|e| format!("OpenAI error: {}", e))?;

        // 9. Try to parse as JSON, fallback to plain text if fails
        let chatbot_response = match serde_json::from_str::<ChatbotResponse>(&response_str) {
            Ok(parsed) => parsed,
            Err(_) => {
                // If JSON parsing fails, treat the entire response as reply
                ChatbotResponse {
                    reply: response_str,
                    actions: vec![],
                }
            }
        };

        // 10. Save assistant reply to database
        self.save_message(user_id, "assistant", &chatbot_response.reply)
            .await?;

        // 11. Execute actions if any
        if !chatbot_response.actions.is_empty() {
            self.execute_actions(user_id, &chatbot_response.actions)
                .await?;
        }

        Ok(chatbot_response)
    }

    /// Build a comprehensive system prompt with user context
    async fn build_system_prompt(
        &self,
        user: &User,
        schedules: &[EatSchedule],
    ) -> Result<String, Box<dyn std::error::Error>> {
        // Format today's schedules
        let mut schedule_info = String::new();
        for schedule in schedules {
            schedule_info.push_str(&format!("- {}: {}\n", schedule.meal_time, schedule.date));
            if let Some(notes) = &schedule.notes {
                schedule_info.push_str(&format!("  Notes: {}\n", notes));
            }
        }

        // Get food preferences info
        let food_prefs_info = if let Some(prefs) = &user.food_preferences {
            format!(
                "Preferred Foods: {}\nAllergies: {}",
                prefs.preferred_foods.join(", "),
                prefs.allergies.join(", ")
            )
        } else {
            "No food preferences set".to_string()
        };

        // Get health info
        let health_info = if let Some(health) = &user.health_profile {
            format!(
                "Height: {} cm\nWeight: {} kg\nMedical Conditions: {}",
                health.height,
                health.weight,
                health.medical_conditions.join(", ")
            )
        } else {
            "No health profile set".to_string()
        };

        let prompt = format!(
            r#"CRITICAL: YOU MUST RESPOND WITH VALID JSON ONLY. NO TEXT OUTSIDE JSON. NO MARKDOWN.

USER INFO:
==========
Username: {username}
Gender: {gender}
Age: {age}
Physical Activity: {activity}
Goal: {goal}
{health_info}

FOOD PREFERENCES:
=================
{food_prefs_info}

TODAY'S SCHEDULES:
==================
{schedule_info}

RULES:
======
1. NEVER recommend foods the user is allergic to: {allergies}
2. Always consider user's goal: {goal}
3. Response must be valid JSON that can be parsed by serde_json
4. If user asks for shopping list, USE create_shopping_schedule ACTION
5. If user asks for reminder, USE create_notification ACTION
6. JSON structure must be EXACTLY as specified below

AVAILABLE ACTIONS:
==================
1. create_notification
2. create_shopping_schedule
3. recommend_meal
4. create_eat_schedule

REQUIRED JSON FORMAT:
=====================
{{
  "reply": "string message here",
  "actions": [  // array of action objects
    {{
      "action_type": "action_name",
      "data": {{  // data specific to each action
        // For create_shopping_schedule:
        "name": "string",
        "items": [
          {{
            "name": "string",
            "quantity": number,
            "unit": "string",
            "category": "string"
          }}
        ]
        // For create_notification:
        "title": "string",
        "message": "string",
        "icon": "string"
        // For recommend_meal:
        "meal_names": ["string"],
        "reason": "string"
        // For create_eat_schedule:
        "date": "string",
        "meal_time": "string", 
        "meal_name": "string",
        "notes": "string"
      }}
    }}
  ]
}}

EXAMPLES:

Example 1 - Shopping list request:
User: "Buatkan shopping list untuk smoothie bowl"
Response: {{
  "reply": "Saya buatkan shopping list untuk smoothie bowl yang sehat!",
  "actions": [
    {{
      "action_type": "create_shopping_schedule",
      "data": {{
        "name": "Bahan Smoothie Bowl",
        "items": [
          {{"name": "Pisang", "quantity": 2, "unit": "buah", "category": "Buah"}},
          {{"name": "Stroberi", "quantity": 250, "unit": "g", "category": "Buah"}},
          {{"name": "Yogurt", "quantity": 200, "unit": "g", "category": "Dairy"}}
        ]
      }}
    }}
  ]
}}

Example 2 - Reminder request:
User: "Ingatkan saya minum air"
Response: {{
  "reply": "Saya akan ingatkan Anda untuk minum air!",
  "actions": [
    {{
      "action_type": "create_notification",
      "data": {{
        "title": "💧 Minum Air",
        "message": "Waktunya minum air!",
        "icon": "💧"
      }}
    }}
  ]
}}

Example 3 - Meal recommendation:
User: "Rekomendasi makan siang yang sehat"
Response: {{
  "reply": "Berikut rekomendasi makan siang yang sehat untuk Anda:",
  "actions": [
    {{
      "action_type": "recommend_meal",
      "data": {{
        "meal_names": ["Salad Ayam", "Quinoa Bowl"],
        "reason": "Tinggi protein dan serat"
      }}
    }}
  ]
}}

Example 4 - Eat schedule request:
User: "Jadwalkan makan malam untuk besok"
Response: {{
  "reply": "Saya jadwalkan makan malam untuk besok!",
  "actions": [
    {{
      "action_type": "create_eat_schedule",
      "data": {{
        "date": "2024-01-16",
        "meal_time": "Dinner",
        "meal_name": "Grilled Salmon",
        "notes": "Dengan sayuran panggang"
      }}
    }}
  ]
}}

IMPORTANT INSTRUCTIONS:
=======================
1. DO NOT wrap response in markdown code blocks
2. DO NOT add any text outside the JSON
3. The JSON MUST be valid and parseable
4. Use actions based on user request
5. Respond in Bahasa Indonesia or English based on user language

NOW RESPOND TO THE USER'S MESSAGE IN VALID JSON FORMAT."#,
            username = user.username,
            gender = user.gender.as_deref().unwrap_or("Not specified"),
            age = user
                .age
                .map(|a| a.to_string())
                .unwrap_or("Not specified".to_string()),
            activity = user
                .physical_activity_level
                .as_deref()
                .unwrap_or("Not specified"),
            goal = user.goal.as_deref().unwrap_or("Not specified"),
            health_info = health_info,
            food_prefs_info = food_prefs_info,
            schedule_info = if schedule_info.is_empty() {
                "No meals scheduled for today".to_string()
            } else {
                schedule_info
            },
            allergies = if let Some(prefs) = &user.food_preferences {
                if prefs.allergies.is_empty() {
                    "None".to_string()
                } else {
                    prefs.allergies.join(", ")
                }
            } else {
                "None".to_string()
            },
        );

        Ok(prompt)
    }

    async fn execute_actions(
        &self,
        user_id: ObjectId,
        actions: &[ChatbotAction],
    ) -> Result<(), Box<dyn std::error::Error>> {
        for action in actions {
            match action {
                ChatbotAction::CreateNotification {
                    title,
                    message,
                    icon,
                } => {
                    self.execute_create_notification(user_id, title, message, icon)
                        .await?;
                }
                ChatbotAction::RecommendMeal { meal_names, reason } => {
                    self.execute_recommend_meal(user_id, meal_names, reason)
                        .await?;
                }
                ChatbotAction::CreateEatSchedule {
                    date,
                    meal_time,
                    meal_name,
                    notes,
                } => {
                    self.execute_create_eat_schedule(user_id, date, meal_time, meal_name, notes)
                        .await?;
                }
                ChatbotAction::CreateShoppingSchedule { name, items } => {
                    self.execute_create_shopping_schedule(user_id, name, items)
                        .await?;
                }
                ChatbotAction::Unknown => {
                    log::warn!("Received unknown action type");
                }
            }
        }
        Ok(())
    }

    async fn execute_create_notification(
        &self,
        user_id: ObjectId,
        title: &str,
        message: &Option<String>,
        icon: &Option<String>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        use crate::models::request::CreateNotificationRequest;

        let request = CreateNotificationRequest {
            title: title.to_string(),
            message: message.clone(),
            icon: icon.clone(),
        };

        self.notification_service
            .create_notification(user_id, request)
            .await?;

        log::info!("Notification created for user {}: {}", user_id, title);
        Ok(())
    }

    async fn execute_recommend_meal(
        &self,
        user_id: ObjectId,
        meal_names: &[String],
        reason: &Option<String>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let message = format!(
            "Meal recommendations: {}. {}",
            meal_names.join(", "),
            reason
                .as_deref()
                .unwrap_or("Suggested based on your profile")
        );

        use crate::models::request::CreateNotificationRequest;

        let request = CreateNotificationRequest {
            title: "🍽️ Meal Recommendations".to_string(),
            message: Some(message),
            icon: Some("🍎".to_string()),
        };

        self.notification_service
            .create_notification(user_id, request)
            .await?;

        log::info!(
            "Meal recommendations sent to user {}: {:?}",
            user_id,
            meal_names
        );
        Ok(())
    }

    async fn execute_create_eat_schedule(
        &self,
        user_id: ObjectId,
        date: &str,
        meal_time: &str,
        meal_name: &str,
        notes: &Option<String>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        use crate::models::meals::Meal;
        use crate::models::request::CreateNotificationRequest;
        use crate::utils::pexels::get_meal_image;

        let user = self
            .user_service
            .get_user_by_id(user_id)
            .await?
            .ok_or("User not found")?;

        let (meal_id, calories) = match self.utils_service.get_meal_by_name(meal_name).await? {
            Some(meal) => (
                meal.id.ok_or("Meal found but has no ID")?,
                meal.calories.unwrap_or(500),
            ),

            None => {
                log::info!("🍽️ Creating AI-generated meal: {}", meal_name);
                let allergies: Vec<String> = user
                    .food_preferences
                    .as_ref()
                    .map(|p| p.allergies.clone())
                    .unwrap_or_default();

                let ingredients = self
                    .generate_ingredients(meal_name, &allergies)
                    .await
                    .unwrap_or_else(|e| {
                        log::error!("Failed to generate ingredients: {}", e);
                        vec!["Main ingredient".to_string()]
                    });

                let goal = user.goal.as_deref().unwrap_or("maintain");

                let calories = self
                    .estimate_calories_ai(meal_name, meal_time, goal)
                    .await
                    .unwrap_or(500);

                let image_url = match get_meal_image(meal_name).await {
                    Ok(Some(url)) => Some(url),
                    Ok(None) => Some(format!(
                        "https://via.placeholder.com/400x300/FF6B6B/FFFFFF?text={}",
                        meal_name.replace(" ", "+")
                    )),
                    Err(_) => Some(format!(
                        "https://via.placeholder.com/400x300/4ECDC4/000000?text={}",
                        meal_name.replace(" ", "+")
                    )),
                };

                let category = if meal_name.to_lowercase().contains("salad") {
                    "salad"
                } else if meal_name.to_lowercase().contains("soup") {
                    "soup"
                } else if meal_name.to_lowercase().contains("smoothie") {
                    "drink"
                } else if meal_name.to_lowercase().contains("grill") {
                    "grilled"
                } else {
                    "main-course"
                };

                let description = format!(
                    "Hidangan {} yang dibuat oleh AI Nutritionist, disesuaikan dengan tujuan {}.",
                    meal_name, goal
                );

                let meal = Meal {
                    id: None,
                    name: meal_name.to_string(),
                    ingredients,
                    category: category.to_string(),
                    calories: Some(calories),
                    image_url,
                    description: Some(description),
                };

                let meal_id = self.utils_service.add_meal(meal).await?;
                (meal_id, calories)
            }
        };

        self.schedule_service
            .create_eat_schedule(
                user_id,
                date.to_string(),
                meal_time.to_string(),
                meal_id,
                notes.clone(),
            )
            .await?;

        let notif = CreateNotificationRequest {
            title: "📅 Meal Scheduled".to_string(),
            message: Some(format!(
                "{} dijadwalkan untuk {} ({} kcal)",
                meal_name, meal_time, calories
            )),
            icon: Some("🍴".to_string()),
        };

        self.notification_service
            .create_notification(user_id, notif)
            .await?;

        log::info!(
            "✅ Eat schedule created: user={}, meal={}, date={}, time={}",
            user_id,
            meal_name,
            date,
            meal_time
        );

        Ok(())
    }

    pub async fn execute_create_shopping_schedule(
        &self,
        user_id: ObjectId,
        name: &str,
        items: &[ScheduleItem],
    ) -> Result<(), Box<dyn std::error::Error>> {
        use crate::models::response::CreateShoppingItem;

        // Convert ScheduleItem to CreateShoppingItem
        let shopping_items: Vec<CreateShoppingItem> = items
            .iter()
            .map(|item| CreateShoppingItem {
                name: item.name.clone(),
                quantity: item.quantity,
                unit: item.unit.clone(),
                category: item.category.clone(),
                notes: None,
            })
            .collect();

        let request = CreateScheduleRequest {
            name: name.to_string(),
            description: Some("Created by chatbot".to_string()),
            items: shopping_items,
        };

        self.schedule_service
            .create_schedule(user_id, request)
            .await?;

        log::info!(
            "Shopping schedule created for user {}: {} with {} items",
            user_id,
            name,
            items.len()
        );

        use crate::models::request::CreateNotificationRequest;

        let notification_request = CreateNotificationRequest {
            title: "🛒 Shopping List Created".to_string(),
            message: Some(format!(
                "Shopping list '{}' created with {} items",
                name,
                items.len()
            )),
            icon: Some("🛒".to_string()),
        };

        self.notification_service
            .create_notification(user_id, notification_request)
            .await?;

        Ok(())
    }

    pub async fn get_chat_history(
        &self,
        user_id: ObjectId,
        limit: i64,
    ) -> mongodb::error::Result<Vec<ChatMessage>> {
        let filter = doc! { "user_id": user_id };
        let find_options = mongodb::options::FindOptions::builder()
            .sort(doc! { "created_at": -1 }) // Get latest first
            .limit(limit)
            .build();

        let cursor = self.chat_collection.find(filter, find_options).await?;
        let mut messages: Vec<ChatMessage> = cursor.try_collect().await?;

        messages.reverse();

        Ok(messages)
    }

    async fn save_message(
        &self,
        user_id: ObjectId,
        role: &str,
        content: &str,
    ) -> mongodb::error::Result<()> {
        let message = ChatMessage {
            id: None,
            user_id,
            role: role.to_string(),
            content: content.to_string(),
            created_at: DateTime::now(),
        };

        self.chat_collection.insert_one(message, None).await?;
        Ok(())
    }

    pub async fn estimate_calories_ai(
        &self,
        meal_name: &str,
        meal_time: &str,
        goal: &str,
    ) -> Result<i32, Box<dyn std::error::Error>> {
        let prompt = format!(
            r#"
You are a professional nutritionist.

Task:
Estimate calories for ONE serving of a meal.

Meal name: "{meal_name}"
Meal time: "{meal_time}"
User goal: "{goal}"

STRICT RULES:
Meal time calorie ranges:
- breakfast: 300–500 kcal
- lunch: 500–750 kcal
- dinner: 400–650 kcal

Goal adjustment:
- cutting → lower range
- maintain → middle range
- bulking → upper range

ASSUME:
- Standard adult portion
- No extreme junk or oversized meals unless clearly implied

OUTPUT:
ONLY valid JSON.
NO text.
NO explanation.

JSON:
{{ "calories": number }}
"#,
        );

        let response = self
            .openai
            .chat(vec![
                json!({ "role": "system", "content": "You are a nutritionist AI." }),
                json!({ "role": "user", "content": prompt }),
            ])
            .await?;

        let parsed: serde_json::Value = serde_json::from_str(&response)
            .map_err(|_| format!("Invalid JSON from AI: {}", response))?;

        let calories = parsed
            .get("calories")
            .and_then(|v| v.as_i64())
            .ok_or("Calories missing")?;

        Ok(calories.clamp(300, 900) as i32)
    }

    pub async fn generate_ingredients(
        &self,
        meal_name: &str,
        allergies: &[String],
    ) -> Result<Vec<String>, Box<dyn std::error::Error>> {
        let prompt = format!(
            r#"
You are a professional nutritionist.

Task:
Generate ingredients for the following meal:
"{meal_name}"

RULES:
- Cuisine can be ANY (Western, Asian, Middle Eastern, etc.)
- Use realistic, commonly available ingredients
- Exclude allergens: {:?}
- MAX 8 ingredients
- NO quantities
- NO explanation

OUTPUT:
ONLY valid JSON

JSON FORMAT:
{{ "ingredients": ["string"] }}
"#,
            allergies
        );

        let response = self
            .openai
            .chat(vec![
                json!({ "role": "system", "content": "You are a nutritionist AI." }),
                json!({ "role": "user", "content": prompt }),
            ])
            .await?;

        let parsed: serde_json::Value = serde_json::from_str(&response)
            .map_err(|_| format!("Invalid JSON from AI: {}", response))?;

        let ingredients = parsed
            .get("ingredients")
            .and_then(|v| v.as_array())
            .ok_or("Ingredients missing")?
            .iter()
            .filter_map(|v| v.as_str().map(|s| s.to_string()))
            .collect::<Vec<_>>();

        Ok(if ingredients.is_empty() {
            vec!["Main ingredient".to_string()]
        } else {
            ingredients
        })
    }

    pub async fn get_recent_conversations(
        &self,
        user_id: ObjectId,
        limit: i64,
    ) -> mongodb::error::Result<Vec<ChatMessage>> {
        let filter = doc! { "user_id": user_id };
        let find_options = mongodb::options::FindOptions::builder()
            .sort(doc! { "created_at": -1 })
            .limit(limit)
            .build();

        let cursor = self.chat_collection.find(filter, find_options).await?;
        cursor.try_collect().await
    }

    pub async fn clear_chat_history(&self, user_id: ObjectId) -> mongodb::error::Result<()> {
        let filter = doc! { "user_id": user_id };
        self.chat_collection.delete_many(filter, None).await?;
        Ok(())
    }
}
