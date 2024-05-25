use chrono::{DateTime, Utc};
use uuid::Uuid;


#[derive(Debug)]
pub struct User {
    pub id: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub username: String,
    pub phone: Option<String>,
    pub email: Option<String>,
    pub phone_verified: bool,
    pub email_verified: bool,
    pub is_admin: bool,
}

#[derive(Debug)]
pub struct UserUnsafe {
    pub id: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub username: String,
    pub password: String,
    pub phone: Option<String>,
    pub email: Option<String>,
    pub phone_verified: bool,
    pub email_verified: bool,
    pub is_admin: bool,
}

#[derive(Debug)]
pub struct UserCreate {
    pub password: String,
    pub username: String,
    pub phone: Option<String>,
    pub email: Option<String>,
    pub phone_verified: bool,
    pub email_verified: bool,
    pub is_admin: bool,
}

#[derive(Debug)]
pub struct UserUpdate {
    pub id: Uuid,
    pub password: String,
    pub username: String,
    pub phone: Option<String>,
    pub email: Option<String>,
    pub phone_verified: bool,
    pub email_verified: bool,
    pub is_admin: bool,
}

#[derive(Debug)]
pub struct UserUpdateSafe {
    pub id: Uuid,
    pub username: String,
    pub phone: Option<String>,
    pub email: Option<String>,
    pub phone_verified: bool,
    pub email_verified: bool,
    pub is_admin: bool,
}