use crate::models::{
    response::{CreateScheduleRequest, CreateShoppingItem},
    schedule::Schedule,
    shoppinglist::ShoppingItem,
};
use futures_util::TryStreamExt;
use mongodb::{
    Collection,
    bson::{doc, oid::ObjectId},
};

#[derive(Clone)]
pub struct ScheduleService {
    pub schedule_collection: Collection<Schedule>,
}

impl ScheduleService {
    pub fn new(schedule_collection: Collection<Schedule>) -> Self {
        Self {
            schedule_collection,
        }
    }

    pub async fn create_schedule(
        &self,
        user_id: ObjectId,
        request: CreateScheduleRequest,
    ) -> Result<Schedule, mongodb::error::Error> {
        println!("🛒 Creating shopping schedule for user: {:?}", user_id);
        println!("📝 Schedule name: {}", request.name);
        println!("📦 Items to add: {}", request.items.len());

        let shopping_list: Vec<ShoppingItem> = request
            .items
            .into_iter()
            .map(|item| ShoppingItem {
                id: None,
                user_id,
                name: item.name,
                quantity: item.quantity,
                unit: item.unit,
                category: item.category,
                purchased: false,
                notes: item.notes,
            })
            .collect();

        let schedule = Schedule {
            id: None,
            user_id,
            name: request.name,
            description: request.description,
            shopping_list,
            created_at: chrono::Utc::now(),
        };

        let result = self.schedule_collection.insert_one(&schedule, None).await?;

        let mut schedule_with_id = schedule;
        schedule_with_id.id = result.inserted_id.as_object_id();

        println!(
            "✅ Shopping schedule '{}' created with {} items",
            schedule_with_id.name,
            schedule_with_id.shopping_list.len()
        );

        Ok(schedule_with_id)
    }

    pub async fn get_user_schedules(
        &self,
        user_id: ObjectId,
    ) -> Result<Vec<Schedule>, mongodb::error::Error> {
        println!("📋 Getting schedules for user: {:?}", user_id);

        let filter = doc! { "user_id": user_id };
        let cursor = self.schedule_collection.find(filter, None).await?;
        let schedules: Vec<Schedule> = cursor.try_collect().await?;

        println!("✅ Found {} schedules", schedules.len());
        Ok(schedules)
    }

    pub async fn get_schedule_by_id(
        &self,
        schedule_id: ObjectId,
        user_id: ObjectId,
    ) -> Result<Option<Schedule>, mongodb::error::Error> {
        println!(
            "🔍 Getting schedule: {:?} for user: {:?}",
            schedule_id, user_id
        );

        let filter = doc! {
            "_id": schedule_id,
            "user_id": user_id
        };
        let schedule = self.schedule_collection.find_one(filter, None).await?;

        match &schedule {
            Some(_) => println!("✅ Schedule found"),
            None => println!("❌ Schedule not found"),
        }

        Ok(schedule)
    }

    pub async fn delete_schedule(
        &self,
        schedule_id: ObjectId,
        user_id: ObjectId,
    ) -> Result<bool, mongodb::error::Error> {
        println!(
            "🗑️ Deleting schedule: {:?} for user: {:?}",
            schedule_id, user_id
        );

        let filter = doc! {
            "_id": schedule_id,
            "user_id": user_id
        };
        let result = self.schedule_collection.delete_one(filter, None).await?;

        let deleted = result.deleted_count > 0;
        if deleted {
            println!("✅ Schedule deleted");
        } else {
            println!("❌ Schedule not found");
        }

        Ok(deleted)
    }

    pub async fn update_shopping_item(
        &self,
        schedule_id: ObjectId,
        item_index: usize,
        user_id: ObjectId,
        purchased: bool,
    ) -> Result<bool, mongodb::error::Error> {
        println!(
            "🔄 Updating item {} in schedule: {:?}",
            item_index, schedule_id
        );

        let filter = doc! {
            "_id": schedule_id,
            "user_id": user_id
        };

        let update = doc! {
            "$set": {
                format!("shoppinglist.{}.purchased", item_index): purchased
            }
        };

        let result = self
            .schedule_collection
            .update_one(filter, update, None)
            .await?;

        let updated = result.modified_count > 0;
        if updated {
            println!("✅ Item updated to purchased: {}", purchased);
        } else {
            println!("❌ Item update failed");
        }

        Ok(updated)
    }

    pub async fn add_item_to_schedule(
        &self,
        schedule_id: ObjectId,
        user_id: ObjectId,
        item: CreateShoppingItem,
    ) -> Result<bool, mongodb::error::Error> {
        println!("➕ Adding item to schedule: {:?}", schedule_id);

        let filter = doc! {
            "_id": schedule_id,
            "user_id": user_id
        };

        let new_shopping_item = ShoppingItem {
            id: None,
            user_id,
            name: item.name,
            quantity: item.quantity,
            unit: item.unit,
            category: item.category,
            purchased: false,
            notes: item.notes,
        };

        let update = doc! {
            "$push": {
                "shoppinglist": mongodb::bson::to_bson(&new_shopping_item)
                    .map_err(|e| mongodb::error::Error::custom(e.to_string()))?
            }
        };

        let result = self
            .schedule_collection
            .update_one(filter, update, None)
            .await?;

        let added = result.modified_count > 0;
        if added {
            println!("✅ Item added to schedule");
        } else {
            println!("❌ Failed to add item");
        }

        Ok(added)
    }

    pub async fn remove_item_from_schedule(
        &self,
        schedule_id: ObjectId,
        user_id: ObjectId,
        item_index: usize,
    ) -> Result<bool, mongodb::error::Error> {
        println!(
            "➖ Removing item {} from schedule: {:?}",
            item_index, schedule_id
        );

        let filter = doc! {
            "_id": schedule_id,
            "user_id": user_id
        };

        let unset_update = doc! {
            "$unset": {
                format!("shoppinglist.{}", item_index): 1
            }
        };

        let pull_update = doc! {
            "$pull": {
                "shoppinglist": null
            }
        };

        let unset_result = self
            .schedule_collection
            .update_one(filter.clone(), unset_update, None)
            .await?;
        if unset_result.modified_count > 0 {
            let pull_result = self
                .schedule_collection
                .update_one(filter, pull_update, None)
                .await?;
            Ok(pull_result.modified_count > 0)
        } else {
            Ok(false)
        }
    }

    pub async fn update_schedule_info(
        &self,
        schedule_id: ObjectId,
        user_id: ObjectId,
        name: Option<String>,
        description: Option<String>,
    ) -> Result<bool, mongodb::error::Error> {
        println!("✏️ Updating schedule info: {:?}", schedule_id);

        let filter = doc! {
            "_id": schedule_id,
            "user_id": user_id
        };

        let mut update_doc = doc! {};

        if let Some(name) = name {
            update_doc.insert("name", name);
        }

        if let Some(description) = description {
            update_doc.insert("description", description);
        }

        if update_doc.is_empty() {
            return Ok(false); // Nothing to update
        }

        let update = doc! { "$set": update_doc };
        let result = self
            .schedule_collection
            .update_one(filter, update, None)
            .await?;

        let updated = result.modified_count > 0;
        if updated {
            println!("✅ Schedule info updated");
        } else {
            println!("❌ Schedule info update failed");
        }

        Ok(updated)
    }
}
