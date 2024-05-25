use chrono::{Duration, Utc};
use jsonwebtoken::{DecodingKey, EncodingKey, Header, TokenData};
use serde::{Deserialize, Serialize};

use crate::entities::user::User;

pub enum JwtType {
    ACCESS,
    REFRESH,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub exp: i64, // Required (validate_exp defaults to true in validation). Expiration time (as UTC timestamp)
    pub iat: i64, // Optional. Issued at (as UTC timestamp)
    pub iss: String, // Optional. Issuer
    pub sub: String, // Optional. Subject (whom token refers to)
    pub adm: bool,
    pub usn: String,
    pub typ: String,
}

pub fn issue_jwt(user: &User, jwt_type: JwtType) -> String {
    let days: i64;
    let mut type_name = String::new();
    match jwt_type {
        JwtType::ACCESS => {
            days = 2;
            type_name.push_str("acc");
        }
        JwtType::REFRESH => {
            days = 14;
            type_name.push_str("rfs");
        }
    }
    let claims = Claims {
        adm: user.is_admin,
        exp: (Utc::now() + Duration::days(days)).timestamp(),
        iat: Utc::now().timestamp(),
        iss: String::from("cloud-user-service"),
        sub: user.id.to_string(),
        usn: String::from(&user.username),
        typ: type_name,
    };
    let secret = std::env::var("SECRET").unwrap();
    jsonwebtoken::encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_str().as_ref()),
    )
    .unwrap()
}

pub fn validate_jwt(token: &str) -> Result<TokenData<Claims>, String> {
    let secret = std::env::var("SECRET").unwrap();
    let token = jsonwebtoken::decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret.as_str().as_ref()),
        &jsonwebtoken::Validation::default(),
    );
    match token {
        Ok(data) => Ok(data),
        Err(_) => Err(String::from("invalid token")),
    }
}
