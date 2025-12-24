mod controller;
mod data;
mod db;
mod handler;
mod middleware;
mod models;
mod services;
mod state;
mod utils;
use axum::{Router, http::Method};
use controller::user_controller::UserController;
use db::mongo::connect;
use dotenv::dotenv;
use mongodb::{Collection, Database};
use state::AppState;
use tokio::net::TcpListener;
use tower_http::cors::{Any, CorsLayer};

use crate::{
    controller::{
        auth_controller::AuthController, notification_controller::NotificationController,
        schedule_controller::ScheduleController,
    },
    utils::seed_meal::smart_seed_meals,
};

#[tokio::main]
async fn main() {
    dotenv().ok();

    // Connect to MongoDB
    let db = connect().await;

    match smart_seed_meals(&db).await {
        Ok(_) => println!("✅ Meal database sync completed"),
        Err(e) => eprintln!("❌ Meal database sync failed: {}", e),
    }
    let state = AppState::new(db);

    let state_for_router = state.clone();

    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods([
            Method::GET,
            Method::POST,
            Method::PUT,
            Method::DELETE,
            Method::OPTIONS,
        ])
        .allow_headers(Any);

    let app = Router::new()
        .merge(UserController::routes())
        .merge(ScheduleController::routes())
        .merge(AuthController::routes())
        .merge(NotificationController::routes())
        .layer(cors)
        .with_state(state_for_router);

    let listener = TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("🚀 Server running at http://localhost:3000");
    println!("🔗 CORS enabled for all origins (development only)");

    let count = state
        .utils_service
        .meals_collection
        .count_documents(None, None)
        .await
        .unwrap();
    println!("🍽️  Database has {} meals", count);

    axum::serve(listener, app).await.unwrap();
}
