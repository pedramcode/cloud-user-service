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
