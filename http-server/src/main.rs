extern crate user_service;

use std::{net::IpAddr, str::FromStr};

pub mod controllers;
pub mod dao;
pub mod utils;

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    dotenvy::dotenv().expect("unable to find \".env\" file");

    let mut http_config = rocket::Config::default();
    http_config.port = match std::env::var("HTTP_PORT") {
        Ok(port) => port.parse::<u16>().unwrap(),
        Err(_) => 8000,
    };
    let addr = match std::env::var("HTTP_HOST") {
        Ok(addr) => addr,
        Err(_) => String::from("127.0.0.1"),
    };
    http_config.address = IpAddr::from_str(addr.as_str()).unwrap();

    let _rocket = rocket::build()
        .configure(http_config)
        .mount(
            "/",
            rocket::routes![controllers::root::root, controllers::root::ping],
        )
        .mount(
            "/api/v1/jwt",
            rocket::routes![
                controllers::jwt::fetch,
                controllers::jwt::refresh,
                controllers::jwt::verify
            ],
        )
        .launch()
        .await?;

    Ok(())
}
