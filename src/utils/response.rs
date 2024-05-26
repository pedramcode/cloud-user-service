use rocket::{
    response::status::Custom,
    serde::{json::Json, Serialize},
};

use crate::dao;

pub type ResponseType<T> = Custom<Json<crate::dao::response::Response<T>>>;

pub fn response<T: Serialize>(
    status: rocket::http::Status,
    message: Option<&str>,
    error: Option<&str>,
    data: Option<T>,
) -> Custom<Json<dao::response::Response<T>>> {
    let resp = dao::response::Response::<T> {
        data: data,
        message: match message {
            Some(x) => Some(String::from(x)),
            None => None,
        },
        error: match error {
            Some(x) => Some(String::from(x)),
            None => None,
        },
    };
    Custom(status, Json(resp))
}
