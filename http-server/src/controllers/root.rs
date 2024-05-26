#[rocket::get("/")]
pub async fn root() -> &'static str {
    "silence is golden"
}

#[rocket::get("/ping")]
pub async fn ping() -> &'static str {
    "PONG"
}
