use actix_web::{web, HttpResponse};
use mongodb::{bson::doc, Collection};
use crate::models::user::User;
use futures::stream::TryStreamExt;
use log::{info, error};
use validator::Validate;

/// Create a new user
pub async fn create_user(
    user: web::Json<User>,
    collection: web::Data<Collection<User>>,
) -> HttpResponse {
    // Validate input
    if let Err(validation_errors) = user.validate() {
        error!("Validation failed: {:?}", validation_errors);
        return HttpResponse::BadRequest().json(validation_errors);
    }

    let mut user = user.into_inner();
    user.id = None; // Ensure the ID is not set by the client

    // Insert the user into the database
    match collection.insert_one(user, None).await {
        Ok(insert_result) => {
            info!("User created: {:?}", insert_result.inserted_id);
            HttpResponse::Created().json(insert_result.inserted_id)
        }
        Err(e) => {
            error!("Failed to create user: {}", e);
            HttpResponse::InternalServerError().body("Failed to create user")
        }
    }
}

/// Get all users
pub async fn get_users(
    collection: web::Data<Collection<User>>,
) -> HttpResponse {
    // Fetch all users from the database
    match collection.find(None, None).await {
        Ok(mut cursor) => {
            let mut users = Vec::new();
            // Collect all users into a vector
            while let Ok(Some(user)) = cursor.try_next().await {
                users.push(user);
            }
            info!("Fetched {} users", users.len());
            HttpResponse::Ok().json(users)
        }
        Err(e) => {
            error!("Failed to fetch users: {}", e);
            HttpResponse::InternalServerError().body("Failed to fetch users")
        }
    }
}

/// Update a user by ID
pub async fn update_user(
    id: web::Path<String>,
    user: web::Json<User>,
    collection: web::Data<Collection<User>>,
) -> HttpResponse {
    // Validate input
    if let Err(validation_errors) = user.validate() {
        error!("Validation failed: {:?}", validation_errors);
        return HttpResponse::BadRequest().json(validation_errors);
    }

    // Parse the ID into an ObjectId
    let oid = match mongodb::bson::oid::ObjectId::parse_str(&*id) {
        Ok(oid) => oid,
        Err(_) => {
            error!("Invalid ID format: {}", id);
            return HttpResponse::BadRequest().body("Invalid ID format");
        }
    };

    // Define the filter and update
    let filter = doc! { "_id": oid };
    let update = doc! { "$set": { "name": user.name.clone(), "email": user.email.clone(), "age": user.age } };

    // Update the user in the database
    match collection.update_one(filter, update, None).await {
        Ok(result) if result.modified_count > 0 => {
            info!("User updated: {}", id);
            HttpResponse::Ok().body("User updated")
        }
        Ok(_) => {
            info!("User not found: {}", id);
            HttpResponse::NotFound().body("User not found")
        }
        Err(e) => {
            error!("Failed to update user: {}", e);
            HttpResponse::InternalServerError().body("Failed to update user")
        }
    }
}

/// Delete a user by ID
pub async fn delete_user(
    id: web::Path<String>,
    collection: web::Data<Collection<User>>,
) -> HttpResponse {
    // Parse the ID into an ObjectId
    let oid = match mongodb::bson::oid::ObjectId::parse_str(&*id) {
        Ok(oid) => oid,
        Err(_) => {
            error!("Invalid ID format: {}", id);
            return HttpResponse::BadRequest().body("Invalid ID format");
        }
    };

    // Define the filter
    let filter = doc! { "_id": oid };

    // Delete the user from the database
    match collection.delete_one(filter, None).await {
        Ok(result) if result.deleted_count > 0 => {
            info!("User deleted: {}", id);
            HttpResponse::Ok().body("User deleted")
        }
        Ok(_) => {
            info!("User not found: {}", id);
            HttpResponse::NotFound().body("User not found")
        }
        Err(e) => {
            error!("Failed to delete user: {}", e);
            HttpResponse::InternalServerError().body("Failed to delete user")
        }
    }
}