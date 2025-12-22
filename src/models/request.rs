use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct UpdateGenderRequest {
    pub gender: String,
}
