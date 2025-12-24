use crate::handler::schedule_handler::{
    add_item_to_schedule_handler, create_eat_schedule_handler, delete_eat_schedule_handler,
    delete_schedule_handler, get_eat_schedule_by_id_handler, get_schedules_by_date_range_handler,
    get_today_schedules_handler, get_user_eat_schedules_handler, get_user_schedules_handler,
    remove_item_from_schedule_handler, update_schedule_info_handler, update_shopping_item_handler,
};
use crate::middleware::authmiddleware::auth_middleware;
use crate::{handler::schedule_handler::create_schedule_handler, state::AppState};
use axum::routing::{delete, put};
use axum::{Router, routing::get, routing::post};

pub struct ScheduleController;

impl ScheduleController {
    pub fn routes() -> Router<AppState> {
        let protected_routes = Router::new()
            .route("/", post(create_schedule_handler))
            .route("/getSchedule", get(get_user_schedules_handler))
            .route("/delete/{id}", delete(delete_schedule_handler))
            .route("/updateItem", put(update_shopping_item_handler))
            .route("/addItem", post(add_item_to_schedule_handler))
            .route("/removeItem", post(remove_item_from_schedule_handler))
            .route("/updateInfo", put(update_schedule_info_handler))
            .route("/eat/create", post(create_eat_schedule_handler))
            .route("/eat", get(get_user_eat_schedules_handler))
            .route("/eat/{id}", get(get_eat_schedule_by_id_handler))
            .route("/eat/today", get(get_today_schedules_handler))
            .route("/eat/range", post(get_schedules_by_date_range_handler))
            .route("/eat/delete/{id}", delete(delete_eat_schedule_handler))
            .layer(axum::middleware::from_fn(auth_middleware));

        Router::new().nest("/schedules", protected_routes)
    }
}
