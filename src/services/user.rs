
use std::str::FromStr;

use chrono::{Duration, Utc};
use uuid::Uuid;

use crate::{entities::{otp::{OtpCreate, OtpMedia, OtpUpdate}, user::{User, UserCreate, UserUpdateSafe}}, repos::{otp::OtpRepo, traits::Crud, user::UserRepo}, utils::{jwt::{issue_jwt, validate_jwt}, otp::generate_otp, security::{hash_password, verify_password}}};


pub struct UserService;

impl UserService {
    pub async fn register(
        username: &str, 
        password: &str, 
        phone: Option<&str>,
        email: Option<&str>,
        is_admin: bool
    ) -> Result<User, String> {
        let pass = hash_password(password);
        let user = UserRepo::create(UserCreate{
            email: match email {Some(dt) => Some(String::from(dt)), None=>None},
            email_verified: false,
            is_admin: is_admin,
            password: pass,
            phone: match phone {Some(dt) => Some(String::from(dt)), None=>None},
            phone_verified: false,
            username: String::from(username),
        }).await?;

        if email != None {
            let otp_res = OtpRepo::create(OtpCreate{
                is_used: false,
                key: generate_otp(),
                r#type: crate::entities::otp::OtpType::Verfiy,
                user_id: user.id,
                media: crate::entities::otp::OtpMedia::Email,
            }).await?;
            // TODO send email
            println!("{:?}", otp_res);
        }

        if phone != None {
            let otp_res = OtpRepo::create(OtpCreate{
                is_used: false,
                key: generate_otp(),
                r#type: crate::entities::otp::OtpType::Verfiy,
                user_id: user.id,
                media: crate::entities::otp::OtpMedia::Phone,
            }).await?;
            // TODO send sms
            println!("{:?}", otp_res);
        }

        Ok(user)
    }

    pub async fn verify_email(otp_key: &str, username: &str) -> Result<User, String> {
        let res = OtpRepo::get_by_key_username(otp_key, username).await?;
        if res.is_used || Utc::now() > res.created_at + Duration::minutes(2) || res.media != OtpMedia::Email {
            return Err(String::from("otp is invalid"));
        }
        let user = UserRepo::get_by_id(res.user_id).await?;
        OtpRepo::update(OtpUpdate{
            id: res.id,
            is_used: true,
            key: res.key,
            media: res.media,
            r#type: res.r#type,
            user_id: res.user_id,
        }).await?;
        let result = UserRepo::update_safe(UserUpdateSafe{
            email: user.email,
            email_verified: true,
            id: user.id,
            is_admin: user.is_admin,
            phone: user.phone,
            phone_verified: user.phone_verified,
            username: user.username,
        }).await?;
        Ok(result)
    }

    pub async fn verify_phone(otp_key: &str, username: &str) -> Result<User, String> {
        let res = OtpRepo::get_by_key_username(otp_key, username).await?;
        if res.is_used || Utc::now() > res.created_at + Duration::minutes(2) || res.media != OtpMedia::Phone {
            return Err(String::from("otp is invalid"));
        }
        let user = UserRepo::get_by_id(res.user_id).await?;
        OtpRepo::update(OtpUpdate{
            id: res.id,
            is_used: true,
            key: res.key,
            media: res.media,
            r#type: res.r#type,
            user_id: res.user_id,
        }).await?;
        let result = UserRepo::update_safe(UserUpdateSafe{
            email: user.email,
            email_verified: user.email_verified,
            id: user.id,
            is_admin: user.is_admin,
            phone: user.phone,
            phone_verified: true,
            username: user.username,
        }).await?;
        Ok(result)
    }

    pub async fn login(username: &str, password: &str) -> Result<(String, String, User), String> {
        let user = UserRepo::get_by_username(username).await?;
        let verified = verify_password(password, &user.password);
        if !verified {
            return Err(String::from("invalid user credentials"));
        }
        let user_res = User {
            created_at: user.created_at,
            email: user.email,
            email_verified: user.email_verified,
            id: user.id,
            is_admin: user.is_admin,
            phone: user.phone,
            phone_verified: user.phone_verified,
            updated_at: user.updated_at,
            username: user.username,
        };
        let access_token = issue_jwt(&user_res, crate::utils::jwt::JwtType::ACCESS);
        let refresh_token = issue_jwt(&user_res, crate::utils::jwt::JwtType::REFRESH);
        Ok((access_token, refresh_token, user_res))
    }

    pub async fn verify(token: &str) -> Result<User, String> {
        let res = validate_jwt(token)?;
        Ok(UserRepo::get_by_id(Uuid::from_str(res.claims.sub.as_str()).unwrap()).await?)
    }
}