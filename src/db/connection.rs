use mongodb::{Client, options::ClientOptions, Database};
use dotenv::dotenv;
use std::env;
use log::{info, error};

pub async fn get_db() -> Result<Database, String> {
    dotenv().ok(); // Load environment variables from .env file

    let user = env::var("MONGO_USER").map_err(|_| "MONGO_USER not set in .env".to_string())?;
    let password = env::var("MONGO_PASSWORD").map_err(|_| "MONGO_PASSWORD not set in .env".to_string())?;
    let host = env::var("MONGO_HOST").map_err(|_| "MONGO_HOST not set in .env".to_string())?;
    let db_name = env::var("MONGO_DB").map_err(|_| "MONGO_DB not set in .env".to_string())?;

    // Construct the connection string securely
    let mongo_uri = format!(
        "mongodb+srv://{}:{}@{}/?retryWrites=true&w=majority&appName=test",
        user, password, host
    );

    let client_options = ClientOptions::parse(&mongo_uri)
        .await
        .map_err(|e| format!("Failed to parse MongoDB URI: {}", e))?;

    let client = Client::with_options(client_options)
        .map_err(|e| format!("Failed to create MongoDB client: {}", e))?;

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
