mod controller;
mod db;
mod handler;
mod middleware;
mod models;
mod services;
mod state;
use crate::services::user_services::UserService;
use crate::state::AppState;
use axum::Router;
use controller::user_controller::UserController;

use tokio::net::TcpListener;

use db::mongo::connect;

#[tokio::main]
async fn main() {
    let db = connect().await;

    let user_service = UserService::new_user_service(&db);
    let state = AppState { db, user_service };

    let app = Router::new()
        .merge(UserController::routes())
        .with_state(state);
    let listener = TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("Server running at 3000...");

    axum::serve(listener, app).await.unwrap();
}
