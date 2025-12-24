use futures_util::TryStreamExt;
use mongodb::{
    Collection,
    bson::{DateTime, doc, oid::ObjectId},
    options::FindOptions,
};

use crate::models::{notification::Notification, request::CreateNotificationRequest};

pub struct NotificationService {
    pub collection: Collection<Notification>,
}
impl NotificationService {
    pub fn new(collection: Collection<Notification>) -> Self {
        Self { collection }
    }
    pub async fn create_notification(
        &self,
        user_id: ObjectId,
        payload: CreateNotificationRequest,
    ) -> mongodb::error::Result<()> {
        let notification = Notification {
            id: None,
            user_id,
            title: payload.title,
            message: payload.message,
            icon: payload.icon,
            is_read: false,
            created_at: DateTime::now(),
        };

        self.collection.insert_one(notification, None).await?;

        Ok(())
    }

    pub async fn get_user_notifications(
        &self,
        user_id: ObjectId,
        limit: i64,
    ) -> mongodb::error::Result<Vec<Notification>> {
        let filter = doc! { "user_id": user_id };
        let options = FindOptions::builder()
            .sort(doc! { "created_at": -1 })
            .limit(limit)
            .build();

        let cursor = self.collection.find(filter, options).await?;
        cursor.try_collect().await
    }

    pub async fn mark_as_read(
        &self,
        notification_id: ObjectId,
        user_id: ObjectId,
    ) -> mongodb::error::Result<bool> {
        let filter = doc! {
            "_id": notification_id,
            "user_id": user_id
        };

        let update = doc! { "$set": { "is_read": true } };
        let result = self.collection.update_one(filter, update, None).await?;
        Ok(result.modified_count > 0)
    }
}
