use crate::models::{
    foodpreferences::FoodPreferences,
    meals::{self, Meal},
    user::User,
};
use futures_util::TryStreamExt;
use mongodb::{
    Collection,
    bson::{Bson, doc, oid::ObjectId, to_bson},
};
#[derive(Clone)]
pub struct UtilsService {
    pub user_collection: Collection<User>,
    pub meals_collection: Collection<Meal>,
}

impl UtilsService {
    pub fn new(user_collection: Collection<User>, meals_collection: Collection<Meal>) -> Self {
        Self {
            user_collection,
            meals_collection,
        }
    }
    //Preferences
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

        Ok(user.and_then(|u| u.food_preferences))
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
    ) -> Result<Vec<Meal>, mongodb::error::Error> {
        let all_meals = self.get_all_meals().await?;

        // Score each meal based on preferences
        let mut scored_meals: Vec<(Meal, i32)> = Vec::new();

        for meal in all_meals {
            let mut score = 0;

            // Score based on preferred foods
            for preferred in &preferences.preferred_foods {
                let preferred_lower = preferred.to_lowercase();

                if meal.name.to_lowercase().contains(&preferred_lower) {
                    score += 3; // High score for name match
                }

                if meal.category.to_lowercase().contains(&preferred_lower) {
                    score += 2; // Medium score for category match
                }

                if meal
                    .ingredients
                    .iter()
                    .any(|ing| ing.to_lowercase().contains(&preferred_lower))
                {
                    score += 1; // Low score for ingredient match
                }
            }

            // Penalize for allergies
            for allergy in &preferences.allergies {
                let allergy_lower = allergy.to_lowercase();

                if meal.name.to_lowercase().contains(&allergy_lower)
                    || meal
                        .ingredients
                        .iter()
                        .any(|ing| ing.to_lowercase().contains(&allergy_lower))
                {
                    score = -100; // Disqualify if contains allergy
                    break;
                }
            }

            if score > 0 {
                scored_meals.push((meal, score));
            }
        }

        // Sort by score (highest first)
        scored_meals.sort_by(|a, b| b.1.cmp(&a.1));

        // Take top 5
        let recommendations: Vec<Meal> = scored_meals
            .into_iter()
            .take(5)
            .map(|(meal, _)| meal)
            .collect();

        let filter = doc! { "_id": user_id };
        let recommendations_bson: Bson =
            to_bson(&recommendations).map_err(|e| mongodb::error::Error::custom(e.to_string()))?;

        let update = doc! {
            "$set": {
                "food_preferences.recommendations": &recommendations_bson,
            }
        };

        match self.user_collection.update_one(filter, update, None).await {
            Ok(result) => {
                println!(
                    "💾 Saved {} meal recommendations for user: {:?} (modified: {})",
                    recommendations.len(),
                    user_id,
                    result.modified_count
                );
            }
            Err(e) => {
                eprintln!("❌ Failed to save recommendations: {}", e);
            }
        }

        Ok(recommendations)
    }

    pub async fn add_meal(&self, meal: Meal) -> Result<ObjectId, mongodb::error::Error> {
        println!("🔧 [UTILS_SERVICE] Adding meal: {:?}", meal);

        let result = self.meals_collection.insert_one(meal, None).await?;

        println!(
            "✅ [UTILS_SERVICE] Meal inserted. ID: {:?}",
            result.inserted_id
        );

        match result.inserted_id.as_object_id() {
            Some(oid) => Ok(oid),
            None => Err(mongodb::error::Error::custom("Failed to get inserted ID")),
        }
    }
    pub async fn get_all_meals(&self) -> Result<Vec<Meal>, mongodb::error::Error> {
        let cursor = self.meals_collection.find(None, None).await?;
        cursor.try_collect().await
    }
}
