mod handler;
mod model;
mod response;
mod route;

//use axum::{response::IntoResponse, routing::get, Json, Router};
use axum::http::{
    header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE},
    HeaderValue, Method,
};
use route::create_router;
use tower_http::cors::CorsLayer;

/*pub async fn server_checker_handler() -> impl IntoResponse {
    const MESSAGE: &str = "Server is up and running";

    let json_response = serde_json::json!({
        "status": "Success",
        "message": MESSAGE
    });

    Json(json_response)
}*/

#[tokio::main]
async fn main() {

    /*let app = Router::new().route("/api/serverchecker", get(server_checker_handler));
    println!("Server started successfully");
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8000").await.unwrap();
    axum::serve(listener, app).await.unwrap();*/
    let cors = CorsLayer::new()
        .allow_origin("http://localhost:3000".parse::<HeaderValue>().unwrap())
        .allow_methods([Method::GET, Method::POST, Method::PATCH, Method::DELETE])
        .allow_credentials(true)
        .allow_headers([AUTHORIZATION, ACCEPT, CONTENT_TYPE]);

    let app = create_router().layer(cors);

    println!("Server started successfully");
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}