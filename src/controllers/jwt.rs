use rocket::serde::json::Json;

use crate::{
    dao::jwt::{FetchRequest, FetchResponse, RefreshRequest, RefreshResponse},
    services::user::UserService,
    utils::{
        jwt::{issue_jwt, JwtType},
        response::{response, ResponseType},
    },
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
    let (access, refresh, _) = res.unwrap();
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
    let res = UserService::verify(&data.refresh_token).await;
    if res.is_err() {
        return response::<RefreshResponse>(
            rocket::http::Status::BadRequest,
            None,
            Some(res.err().unwrap().as_str()),
            None,
        );
    }
    let (user, payload) = res.unwrap();
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
