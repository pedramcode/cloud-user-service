use rocket::serde::Serialize;

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct Response<T: Serialize> {
    pub message: Option<String>,
    pub data: Option<T>,
    pub error: Option<String>,
}
