use crate::models::{foodpreferences::FoodPreferences, user::User};
use mongodb::{
    Collection,
    bson::{doc, oid::ObjectId},
};

#[derive(Clone)]
pub struct UtilsService {
    pub user_collection: Collection<User>,
}

impl UtilsService {
    pub fn new(user_collection: Collection<User>) -> Self {
        Self { user_collection }
    }

    pub async fn add_food_preferences(
        &self,
        user_id: ObjectId,
        foods: FoodPreferences,
    ) -> Result<(), mongodb::error::Error> {
        let filter = doc! { "_id": user_id };
        let user_exists = self.user_collection.find_one(filter.clone(), None).await?;

        if user_exists.is_none() {
            return Err(mongodb::error::Error::custom("User not found"));
        }

        let update = doc! {
            "$set": {
                "preferred_foods": foods.preferred_foods,
                "allergies": foods.allergies
            }
        };

        let result = self
            .user_collection
            .update_one(filter, update, None)
            .await?;
        println!(
            "Updated {} document for user: {:?}",
            result.modified_count, user_id
        );
        Ok(())
    }

    pub async fn remove_preferences(&self, user_id: ObjectId) -> Result<(), mongodb::error::Error> {
        let filter = doc! { "_id": user_id };

        // Update untuk menghapus food preferences (set ke array kosong)
        let update = doc! {
            "$set": {
                "food_preferences.preferred_foods": [],
                "food_preferences.allergies": []
            }
        };

        let result = self
            .user_collection
            .update_one(filter, update, None)
            .await?;

        if result.modified_count == 0 {
            return Err(mongodb::error::Error::custom(
                "User not found or no preferences to remove",
            ));
        }

        println!("Removed food preferences for user: {:?}", user_id);
        Ok(())
    }
    pub async fn get_user_food_preferences(
        &self,
        user_id: ObjectId,
    ) -> Result<Option<FoodPreferences>, mongodb::error::Error> {
        let filter = doc! { "_id": user_id };
        let user = self.user_collection.find_one(filter, None).await?;

        match user {
            Some(user) => Ok(Some(FoodPreferences {
                preferred_foods: user.food_preferences.preferred_foods,
                allergies: user.food_preferences.allergies,
                recommendations: user.food_preferences.recommendations,
            })),
            None => Ok(None),
        }
    }

    pub async fn update_allergies(
        &self,
        user_id: ObjectId,
        allergies: Vec<String>,
    ) -> Result<(), mongodb::error::Error> {
        let filter = doc! { "_id": user_id };
        let update = doc! { "$set": { "allergies": allergies } };

        self.user_collection
            .update_one(filter, update, None)
            .await?;
        Ok(())
    }
    pub async fn generate_recommend(
        &self,
        user_id: ObjectId,
        preferences: &FoodPreferences,
    ) -> Vec<String> {
        let filter = doc! { "_id": user_id };
        let meal_database = vec![
            (
                "nasi goreng",
                vec!["Nasi Goreng Special", "Nasi Goreng Seafood"],
            ),
            ("ayam", vec!["Ayam Bakar", "Ayam Goreng", "Ayam Panggang"]),
            ("ikan", vec!["Ikan Bakar", "Ikan Goreng", "Ikan Kukus"]),
            ("sayur", vec!["Capcay", "Tumis Sayur", "Sayur Asem"]),
            ("sate", vec!["Sate Ayam", "Sate Kambing"]),
            ("rendang", vec!["Rendang Sapi", "Rendang Ayam"]),
        ];

        let mut recommendations = Vec::new();
        for preferred in &preferences.preferred_foods {
            let preferred_lower = preferred.to_lowercase();

            for (category, meals) in &meal_database {
                if preferred_lower.contains(category) {
                    for meal in meals {
                        let mut safe = true;
                        for allergy in &preferences.allergies {
                            if meal.to_lowercase().contains(&allergy.to_lowercase()) {
                                safe = false;
                                break;
                            }
                        }

                        if safe {
                            recommendations.push(meal.to_string());
                        }
                    }
                }
            }
        }
        let update = doc! {
            "$set": {
                "food_preferences.recommendations": &recommendations
            }
        };

        // Remove duplicates
        recommendations.sort();
        recommendations.dedup();

        recommendations.truncate(5);

        match self.user_collection.update_one(filter, update, None).await {
            Ok(result) => {
                println!(
                    "💾 Saved {} recommendations (modified: {})",
                    recommendations.len(),
                    result.modified_count
                );
            }
            Err(e) => {
                eprintln!("❌ Failed to save recommendations: {}", e);
            }
        }
        recommendations
    }
}
