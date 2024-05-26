use rocket::serde::{Deserialize, Serialize};

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct FetchRequest {
    pub username: String,
    pub password: String,
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct FetchResponse {
    pub access_token: String,
    pub refresh_token: String,
}

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct RefreshRequest {
    pub refresh_token: String,
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct RefreshResponse {
    pub access_token: String,
}

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct VerifyRequest {
    pub access_token: String,
}
