use actix_web::{web, App, HttpServer};
use mongodb::Collection;
use crate::models::user::User;
use crate::db::connection::get_db;
use crate::handlers::user_handlers::{create_user, get_users, update_user, delete_user};
use log::info;
use env_logger;

// Import modules
mod models;
mod db;
mod handlers;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Initialize logger with hardcoded default level
    env_logger::Builder::new()
        .filter_level(log::LevelFilter::Info) // Hardcoded "info" level
        .init();
    
    info!("Starting server...");

    // Connect to MongoDB
    let db = get_db().await.expect("Failed to connect to MongoDB");
    let collection: Collection<User> = db.collection("users");

    // Start the Actix web server
    HttpServer::new(move || {
        App::new()
            // Share the MongoDB collection with all handlers
            .app_data(web::Data::new(collection.clone()))
            // Register routes
            .route("/users", web::post().to(create_user))
            .route("/users", web::get().to(get_users))
            .route("/users/{id}", web::put().to(update_user))
            .route("/users/{id}", web::delete().to(delete_user))
    })
    .bind("0.0.0.0:5050")?
    .run()
    .await
}