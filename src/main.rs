mod handler;
mod model;
mod response;
mod route;

use axum::http::{
    header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE},
    HeaderValue, Method,
};
use route::create_router;
use tower_http::cors::CorsLayer;
use std::sync::Arc;
use dotenv::dotenv;
use sqlx::mysql::{MySqlPool, MySqlPoolOptions};

#[tokio::main]
async fn main() {
    dotenv().ok();
    println!("REST API Service");

    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must set");
    let pool = match MySqlPoolOptions::new()
        .max_connections(10)
        .connect(&database_url)
        .await
        {
            Ok(pool) => {
                println!("Connection to database is successful");
                pool
            }
            Err(err) => {
                println!("Connection to database failed: {:?}", err);
                std::process::exit(1);
            }
        };

    let cors = CorsLayer::new()
        .allow_origin("http://localhost:3000".parse::<HeaderValue>().unwrap())
        .allow_methods([Method::GET, Method::POST, Method::PATCH, Method::DELETE])
        .allow_credentials(true)
        .allow_headers([AUTHORIZATION, ACCEPT, CONTENT_TYPE]);

    let app = create_router(pool).layer(cors);

    println!("Server started successfully");
    let listener = tokio::net::TcpListener::bind("127.0.0.1:8000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}