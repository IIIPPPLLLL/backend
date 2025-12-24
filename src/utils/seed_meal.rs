use mongodb::{Collection, Database, bson::doc};

use crate::{data::seed_meals, models::meals::Meal};
pub async fn smart_seed_meals(db: &Database) -> Result<(), Box<dyn std::error::Error>> {
    let collection: Collection<Meal> = db.collection("meals");

    println!("🤖 Smart meal seeding...");

    let count = collection.count_documents(None, None).await?;

    if count == 0 {
        println!("📦 First-time seeding...");
        let meals = seed_meals::get_seed_meals().await;
        let meals_len = meals.len(); // Simpan length sebelum move

        collection.insert_many(meals, None).await?;
        println!("✅ Seeded {} meals", meals_len);
    } else {
        println!("🔍 Database has {} meals, checking for updates...", count);

        let seed_meals = seed_meals::get_seed_meals().await;
        let existing_names: Vec<String> = collection
            .distinct("name", None, None)
            .await?
            .into_iter()
            .filter_map(|v| v.as_str().map(|s| s.to_string()))
            .collect();

        let (to_update, to_add): (Vec<Meal>, Vec<Meal>) = seed_meals
            .into_iter()
            .partition(|meal| existing_names.contains(&meal.name));

        // Update existing
        for meal in &to_update {
            let filter = doc! { "name": &meal.name };
            let update = doc! {
                "$set": {
                    "ingredients": &meal.ingredients,
                    "category": &meal.category,
                    "calories": meal.calories,
                    "image_url": &meal.image_url,
                    "description": &meal.description
                }
            };
            collection.update_one(filter, update, None).await?;
        }

        // Add new
        if !to_add.is_empty() {
            let to_add_len = to_add.len();
            collection.insert_many(to_add.clone(), None).await?;
            println!("✅ Added {} new meals:", to_add_len);
            for meal in &to_add {
                println!("   ➕ {}", meal.name);
            }
        }

        if !to_update.is_empty() {
            println!("🔄 Updated {} existing meals", to_update.len());
        }
    }

    Ok(())
}
