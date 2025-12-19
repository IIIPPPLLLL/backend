use crate::models::{claims::Claims, loginrequest::LoginRequest, user::User}; // Import Claims dari models
use crate::state::AppState;
use axum::http::StatusCode;
use axum::{Json, extract::State, response::IntoResponse};
use chrono::{Duration, Utc};
use jsonwebtoken::{EncodingKey, Header, encode};
use serde::{Deserialize, Serialize};
use serde_json::json;

pub async fn login(
    State(state): State<AppState>,
    Json(payload): Json<LoginRequest>,
) -> impl IntoResponse {
    let user = match state
        .user_service
        .login_user(payload.username, payload.password)
        .await
    {
        Ok(Some(user)) => user,
        Ok(None) => {
            return (
                StatusCode::UNAUTHORIZED,
                Json(json!({
                    "message": "Invalid credentials",
                    "data": null
                })),
            );
        }
        Err(e) => {
            eprintln!("Login database error: {:?}", e);
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({
                    "message": "Database error",
                    "data": null
                })),
            );
        }
    };

    let user_id = user.id.unwrap().to_string();

    let expiration = Utc::now()
        .checked_add_signed(Duration::hours(24))
        .expect("valid timestamp")
        .timestamp() as usize;

    let claims = Claims {
        user_id: user_id.clone(),
        email: user.email.clone(),
        exp: expiration,
    };

    let secret = std::env::var("JWT_SECRET").unwrap_or_else(|_| "secret".to_string());

    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_ref()),
    )
    .unwrap();

    (
        StatusCode::OK,
        Json(json!({
            "message": "Login successful",
            "data": {
                "token": token,
                "user_id": user_id,
                "email": user.email
            }
        })),
    )
}

pub async fn register_user(
    State(state): State<AppState>,
    Json(payload): Json<User>,
) -> impl IntoResponse {
    match state.user_service.register_user(payload).await {
        Ok(_) => (
            StatusCode::CREATED,
            Json(json!({
                "message": "User registered successfully",
                "data": null
            })),
        ),
        Err(e) => {
            eprintln!("Registration error: {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({
                    "message": "Registration failed",
                    "data": null
                })),
            )
        }
    }
}
