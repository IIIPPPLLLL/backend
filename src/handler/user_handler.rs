use crate::models::user::User;
use crate::state::AppState;
use axum::{Extension, Json, extract::State};
use mongodb::bson::oid::ObjectId;

// pub async fn get_profile(
//     Extension(user_id): Extension<ObjectId>,
//     State(state): State<AppState>,
// ) -> Json<User> {
//     let user = state.user_service.get_user_by_id(user_id).await.unwrap();
//     Json(user)
// }
