use mongodb::{Client, options::ClientOptions, Database};
use dotenv::dotenv;
use std::env;
use log::{info, error};

pub async fn get_db() -> Result<Database, String> {
    dotenv().ok(); // Load environment variables from .env file

    let mongo_uri = env::var("MONGO_URI").map_err(|_| "MONGO_URI not set in .env".to_string())?;
    let db_name = env::var("DATABASE_NAME").map_err(|_| "DATABASE_NAME not set in .env".to_string())?;

    // Parse MongoDB connection options
    let client_options = ClientOptions::parse(&mongo_uri)
        .await
        .map_err(|e| format!("Failed to parse MongoDB URI: {}", e))?;

    // Create MongoDB client
    let client = Client::with_options(client_options)
        .map_err(|e| format!("Failed to create MongoDB client: {}", e))?;

    // Verify the connection by pinging the database
    match client.list_database_names(None, None).await {
        Ok(_) => {
            info!("Successfully connected to MongoDB!");
            Ok(client.database(&db_name))
        }
        Err(e) => {
            error!("Failed to connect to MongoDB: {}", e);
            Err(format!("Failed to connect to MongoDB: {}", e))
        }
    }
}