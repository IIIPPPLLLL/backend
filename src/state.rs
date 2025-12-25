use crate::{
    models::{
        chatmessage::ChatMessage,
        eatschedule::EatSchedule,
        meals::Meal,
        notification::{self, Notification},
        schedule::Schedule,
        user::User,
    },
    services::{
        chatbot_service::ChatbotService, notification_service::NotificationService,
        schedule_services::ScheduleService, user_services::UserService,
        utils_service::UtilsService,
    },
    utils::llm::openai_client::OpenAiClient,
};
use mongodb::Database;
use std::{env, sync::Arc};

#[derive(Clone)]
pub struct AppState {
    pub db: Database,
    pub user_service: Arc<UserService>,
    pub utils_service: Arc<UtilsService>,
    pub schedule_service: Arc<ScheduleService>,
    pub notification_service: Arc<NotificationService>,
    pub chatbot_service: Arc<ChatbotService>,
}
impl AppState {
    pub fn new(db: Database) -> Self {
        let openai_key = std::env::var("OPENAI_API_KEY").expect("OPENAI_API_KEY not set");

        let openai_client = OpenAiClient::new(openai_key);

        let user_collection = db.collection::<User>("users");
        let meals_collection = db.collection::<Meal>("meals");
        let schedule_collection = db.collection::<Schedule>("schedules");
        let eat_schedule_collection = db.collection::<EatSchedule>("eatschedules");
        let notification_collection = db.collection::<Notification>("notifications");
        let chat_collection = db.collection::<ChatMessage>("chat_messages");

        let user_service = Arc::new(UserService::new(user_collection.clone()));
        let utils_service = Arc::new(UtilsService::new(user_collection, meals_collection.clone()));
        let schedule_service = Arc::new(ScheduleService::new(
            schedule_collection,
            eat_schedule_collection,
        ));
        let notification_service = Arc::new(NotificationService::new(notification_collection));
        let chatbot_service = Arc::new(ChatbotService::new(
            chat_collection,
            openai_client,
            user_service.clone(),
            schedule_service.clone(),
            notification_service.clone(),
        ));
        Self {
            db,
            user_service,
            utils_service,
            schedule_service,
            notification_service,
            chatbot_service,
        }
    }
}
