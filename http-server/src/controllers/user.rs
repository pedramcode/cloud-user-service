use crate::{
    dao::{jwt::FetchResponse, user::RegisterRequest},
    utils::{
        jwt::{issue_jwt, JwtType},
        response::{response, ResponseType},
    },
};
use rocket::serde::json::Json;
use user_service::services::user::UserService;

#[rocket::post("/register", data = "<data>")]
pub async fn register(data: Json<RegisterRequest>) -> ResponseType<FetchResponse> {
    let res = UserService::register(
        &data.username,
        &data.password,
        match &data.phone {
            Some(x) => Some(x.as_str()),
            None => None,
        },
        match &data.email {
            Some(x) => Some(x.as_str()),
            None => None,
        },
        data.is_admin,
    )
    .await;
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

#[rocket::put("/verify/email/<username>/<otp>")]
pub async fn verify_email(username: String, otp: String) -> ResponseType<()> {
    let res = UserService::verify_email(otp.as_str(), username.as_str()).await;
    match res {
        Ok(_) => response::<()>(rocket::http::Status::Ok, Some("email verified"), None, None),
        Err(err) => response::<()>(
            rocket::http::Status::BadRequest,
            None,
            Some(err.as_str()),
            None,
        ),
    }
}

#[rocket::put("/verify/phone/<username>/<otp>")]
pub async fn verify_phone(username: String, otp: String) -> ResponseType<()> {
    let res = UserService::verify_phone(otp.as_str(), username.as_str()).await;
    match res {
        Ok(_) => response::<()>(rocket::http::Status::Ok, Some("phone verified"), None, None),
        Err(err) => response::<()>(
            rocket::http::Status::BadRequest,
            None,
            Some(err.as_str()),
            None,
        ),
    }
}
