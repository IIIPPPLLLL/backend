mod controller;
mod data;
mod db;
mod handler;
mod middleware;
mod models;
mod services;
mod state;

use axum::{Router, http::Method};
use controller::user_controller::UserController;
use db::mongo::connect;
use dotenv::dotenv;
use mongodb::{Collection, Database};
use state::AppState;
use tokio::net::TcpListener;
use tower_http::cors::{Any, CorsLayer}; // <- TAMBAHKAN INI

use crate::{controller::schedule_controller::ScheduleController, data::seed_meals};
use models::meals::Meal;

#[tokio::main]
async fn main() {
    dotenv().ok();

    // Connect to MongoDB
    let db = connect().await;

    seed_if_empty(&db).await;

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
async fn seed_if_empty(db: &Database) {
    let collection: Collection<Meal> = db.collection("meals");

    let count = match collection.count_documents(None, None).await {
        Ok(count) => count,
        Err(_) => {
            println!("⚠️  Could not check meals collection");
            return;
        }
    };

    if count == 0 {
        println!("🌱 Seeding meals database...");

        let meals = seed_meals::get_seed_meals();

        let chunks = meals.chunks(10);
        for chunk in chunks {
            if let Err(e) = collection.insert_many(chunk.to_vec(), None).await {
                eprintln!("❌ Failed to insert chunk: {}", e);
            }
        }

        println!("✅ Seeded {} meals", meals.len());
    } else {
        println!("📊 Meals collection already has {} entries", count);
    }
}
