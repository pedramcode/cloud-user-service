use std::str::FromStr;

use rocket::serde::json::Json;
use uuid::Uuid;

use crate::{
    dao::{
        jwt::{FetchRequest, FetchResponse, RefreshRequest, RefreshResponse, VerifyRequest},
        user::User,
    },
    utils::{
        jwt::{issue_jwt, validate_jwt, JwtType},
        response::{response, ResponseType},
    },
};

use user_service::{
    repos::{traits::Crud, user::UserRepo},
    services::user::UserService,
};

#[rocket::post("/fetch", data = "<data>")]
pub async fn fetch(data: Json<FetchRequest>) -> ResponseType<FetchResponse> {
    let res = UserService::login(&data.username, &data.password).await;
    if res.is_err() {
        return response::<FetchResponse>(
            rocket::http::Status::BadRequest,
            None,
            Some(res.err().unwrap().as_str()),
            None,
        );
    }
    let user = res.unwrap();
    let access = issue_jwt(&user, JwtType::ACCESS);
    let refresh = issue_jwt(&user, JwtType::REFRESH);
    return response::<FetchResponse>(
        rocket::http::Status::Ok,
        None,
        None,
        Some(FetchResponse {
            access_token: access,
            refresh_token: refresh,
        }),
    );
}

#[rocket::post("/refresh", data = "<data>")]
pub async fn refresh(data: Json<RefreshRequest>) -> ResponseType<RefreshResponse> {
    let payload = validate_jwt(&data.refresh_token);
    if payload.is_err() {
        return response::<RefreshResponse>(
            rocket::http::Status::BadRequest,
            None,
            Some("invalid token"),
            None,
        );
    }
    let payload = payload.unwrap();

    let uid = Uuid::from_str(&payload.claims.sub).unwrap();
    let user = UserRepo::get_by_id(uid).await;
    if user.is_err() {
        return response::<RefreshResponse>(
            rocket::http::Status::BadRequest,
            None,
            Some(user.err().unwrap().as_str()),
            None,
        );
    }

    let user = user.unwrap();
    if JwtType::from_str(&payload.claims.typ) != JwtType::REFRESH {
        return response::<RefreshResponse>(
            rocket::http::Status::BadRequest,
            None,
            Some("invalid refresh token"),
            None,
        );
    }
    let token = issue_jwt(&user, JwtType::ACCESS);
    response::<RefreshResponse>(
        rocket::http::Status::Ok,
        None,
        None,
        Some(RefreshResponse {
            access_token: token,
        }),
    )
}

#[rocket::post("/verify", data = "<data>")]
pub async fn verify(data: Json<VerifyRequest>) -> ResponseType<User> {
    let payload = validate_jwt(&data.access_token);
    if payload.is_err() {
        return response::<User>(
            rocket::http::Status::BadRequest,
            None,
            Some("invalid token"),
            None,
        );
    }
    let payload = payload.unwrap();

    if JwtType::from_str(&payload.claims.typ) != JwtType::ACCESS {
        return response::<User>(
            rocket::http::Status::BadRequest,
            None,
            Some("invalid access token"),
            None,
        );
    }

    let uid = Uuid::from_str(&payload.claims.sub).unwrap();
    let user = UserRepo::get_by_id(uid).await;
    if user.is_err() {
        return response::<User>(
            rocket::http::Status::BadRequest,
            None,
            Some(user.err().unwrap().as_str()),
            None,
        );
    }
    let user = user.unwrap();

    return response::<User>(
        rocket::http::Status::Ok,
        None,
        None,
        Some(User {
            created_at: user.created_at.timestamp(),
            updated_at: user.updated_at.timestamp(),
            email: user.email,
            phone: user.phone,
            email_verified: user.email_verified,
            is_admin: user.is_admin,
            phone_verified: user.phone_verified,
            id: user.id.to_string(),
            username: user.username,
        }),
    );
}
