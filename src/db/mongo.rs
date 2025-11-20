use mongodb::{Client, Database};

pub async fn connect() -> Database {
    let client = Client::with_uri_str(
        "mongodb+srv://foxyninenineee_db_user:akame112@fitapp.zqykncy.mongodb.net/?retryWrites=true&w=majority",
    )
    .await
    .expect("Failed to connect Mongo");

    client.database("fitApp")
}
