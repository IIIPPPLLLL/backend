use crate::models::claims::Claims;
use axum::{extract::Request, http::StatusCode, middleware::Next, response::Response};
use jsonwebtoken::{Algorithm, DecodingKey, Validation, decode};
use mongodb::bson::oid::ObjectId; // Import dari file yang sama

pub async fn auth_middleware(mut request: Request, next: Next) -> Result<Response, StatusCode> {
    let token = request
        .headers()
        .get("authorization")
        .and_then(|header| header.to_str().ok())
        .and_then(|header| header.strip_prefix("Bearer "))
        .ok_or(StatusCode::UNAUTHORIZED)?;

    let token_data = decode::<Claims>(
        token,
        &DecodingKey::from_secret("your-secret-key".as_ref()),
        &Validation::new(Algorithm::HS256),
    )
    .map_err(|_| StatusCode::UNAUTHORIZED)?;

    // Convert string user_id back to ObjectId
    let user_id =
        ObjectId::parse_str(&token_data.claims.user_id).map_err(|_| StatusCode::UNAUTHORIZED)?;

    request.extensions_mut().insert(user_id);

    Ok(next.run(request).await)
}
