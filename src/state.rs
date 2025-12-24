use crate::{
    models::{
        eatschedule::EatSchedule,
        meals::Meal,
        notification::{self, Notification},
        schedule::Schedule,
        user::User,
    },
    services::{
        notification_service::NotificationService, schedule_services::ScheduleService,
        user_services::UserService, utils_service::UtilsService,
    },
};
use mongodb::Database;
use std::sync::Arc;

#[derive(Clone)]
pub struct AppState {
    pub db: Database,
    pub user_service: Arc<UserService>,
    pub utils_service: Arc<UtilsService>,
    pub schedule_service: Arc<ScheduleService>,
    pub notification_service: Arc<NotificationService>,
}
impl AppState {
    pub fn new(db: Database) -> Self {
        let user_collection = db.collection::<User>("users");
        let meals_collection = db.collection::<Meal>("meals");
        let schedule_collection = db.collection::<Schedule>("schedules");
        let eat_schedule_collection = db.collection::<EatSchedule>("eatschedules");
        let notification_collection = db.collection::<Notification>("notifications");

        let user_service = Arc::new(UserService::new(user_collection.clone()));
        let utils_service = Arc::new(UtilsService::new(user_collection, meals_collection.clone()));
        let schedule_service = Arc::new(ScheduleService::new(
            schedule_collection,
            eat_schedule_collection,
        ));
        let notification_service = Arc::new(NotificationService::new(notification_collection));
        Self {
            db,
            user_service,
            utils_service,
            schedule_service,
            notification_service,
        }
    }
}
