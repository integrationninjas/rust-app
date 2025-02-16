use serde::{Deserialize, Serialize};
use mongodb::bson::oid::ObjectId;
use validator::Validate;

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct User {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    #[validate(length(min = 3, message = "Name must be at least 3 characters"))]
    pub name: String,
    #[validate(email(message = "Email must be a valid email address"))]
    pub email: String,
    #[validate(range(min = 1, max = 120, message = "Age must be between 1 and 120"))]
    pub age: i32,
}