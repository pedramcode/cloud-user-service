pub mod config;
pub mod entities;
pub mod repos;
pub mod services;
pub mod utils;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().expect("unable to find \".env\" file");
}
