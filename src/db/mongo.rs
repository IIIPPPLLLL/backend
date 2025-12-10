use mongodb::{Client, Database};

pub async fn connect() -> Database {
    let uri =
        "mongodb+srv://foxyninenineee_db_user:akame112@ippl.xhbnvki.mongodb.net/?appName=ippl";

    let client = Client::with_uri_str(uri)
        .await
        .expect("Failed to connect Mongo");

    // TEST LIST DATABASES
    match client.list_database_names(None, None).await {
        Ok(dbs) => println!("📊 Available databases: {:?}", dbs),
        Err(e) => eprintln!("❌ Cannot list databases: {}", e),
    }

    let db = client.database("fitApp");

    // TEST LIST COLLECTIONS
    match db.list_collection_names(None).await {
        Ok(collections) => println!("📁 Available collections: {:?}", collections),
        Err(e) => eprintln!("❌ Cannot list collections: {}", e),
    }

    println!("✅ MongoDB connected successfully!");
    db
}
