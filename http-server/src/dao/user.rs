use rocket::serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct User {
    pub id: String,
    pub created_at: i64,
    pub updated_at: i64,
    pub username: String,
    pub phone: Option<String>,
    pub email: Option<String>,
    pub phone_verified: bool,
    pub email_verified: bool,
    pub is_admin: bool,
}

#[derive(Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct RegisterRequest {
    pub username: String,
    pub password: String,
    pub phone: Option<String>,
    pub email: Option<String>,
    pub is_admin: bool,
}
