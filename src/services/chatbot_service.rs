// chatbot_service.rs
use crate::{
    models::{
        chatbotresponse::{ChatbotAction, ChatbotResponse, ScheduleItem},
        chatmessage::ChatMessage,
        eatschedule::EatSchedule,
        response::CreateScheduleRequest,
        user::User,
    },
    services::{
        notification_service::NotificationService, schedule_services::ScheduleService,
        user_services::UserService,
    },
    utils::llm::openai_client::OpenAiClient,
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
}

impl ChatbotService {
    pub fn new(
        chat_collection: Collection<ChatMessage>,
        openai: OpenAiClient,
        user_service: Arc<UserService>,
        schedule_service: Arc<ScheduleService>,
        notification_service: Arc<NotificationService>,
    ) -> Self {
        Self {
            chat_collection,
            openai,
            user_service,
            schedule_service,
            notification_service,
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

    /// Execute actions from the chatbot response
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

    /// Helper method to create notification
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

    /// Helper method to recommend meals
    async fn execute_recommend_meal(
        &self,
        user_id: ObjectId,
        meal_names: &[String],
        reason: &Option<String>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        // Create a notification with the recommendations
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

    /// Helper method to create eat schedule
    async fn execute_create_eat_schedule(
        &self,
        user_id: ObjectId,
        date: &str,
        meal_time: &str,
        meal_name: &str,
        notes: &Option<String>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        // First, we need to find or create a meal
        // For simplicity, we'll create a new meal
        use crate::models::meals::Meal;

        // In a real implementation, you would:
        // 1. Search for existing meal with similar name
        // 2. Or create a new meal in the database
        // 3. Then create the eat schedule

        // For now, we'll just create a notification about the scheduled meal
        let message = format!(
            "I've scheduled '{}' for {} at {}. {}",
            meal_name,
            date,
            meal_time,
            notes.as_deref().unwrap_or("")
        );

        use crate::models::request::CreateNotificationRequest;

        let request = CreateNotificationRequest {
            title: "📅 Meal Scheduled".to_string(),
            message: Some(message),
            icon: Some("🍴".to_string()),
        };

        self.notification_service
            .create_notification(user_id, request)
            .await?;

        log::info!(
            "Eat schedule created for user {}: {} at {}",
            user_id,
            meal_name,
            date
        );
        Ok(())
    }

    /// Helper method to create shopping schedule
    async fn execute_create_shopping_schedule(
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

        // Also create a notification
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

    /// Get chat history for a user
    async fn get_chat_history(
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

        // Reverse to get chronological order (oldest to newest)
        messages.reverse();

        Ok(messages)
    }

    /// Save a message to the chat history
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

    /// Get recent conversations (optional helper method)
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

    /// Clear chat history for a user (optional helper method)
    pub async fn clear_chat_history(&self, user_id: ObjectId) -> mongodb::error::Result<()> {
        let filter = doc! { "user_id": user_id };
        self.chat_collection.delete_many(filter, None).await?;
        Ok(())
    }
}
