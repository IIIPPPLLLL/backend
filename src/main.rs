// main.rs - CLEAN
mod controller;
mod db;
mod handler;
mod middleware;
mod models;
mod services;
mod state;

use axum::Router;
use controller::user_controller::UserController;
use db::mongo::connect;
use state::AppState;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    let db = connect().await;
    let state = AppState::new(db);

    let app = Router::new()
        .merge(UserController::routes())
        .with_state(state);

    let listener = TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("🚀 Server running at http://localhost:3000");

    axum::serve(listener, app).await.unwrap();
}
