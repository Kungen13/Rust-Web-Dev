mod question_f;
mod api;

use question_f::*;
use api::*;
//use std::str::FromStr;

use axum::{response::IntoResponse, routing::get, Json, Router};

/*async fn health_checker_handler() -> impl IntoResponse {
    //(StatusCode::NOT_FOUND, "404 Not Found").into_response()
    const MESSAGE: &str = "Build Simple CRUD API in Rust using Axum";
    let json_response = serde_json::json!({
        "status": "success",
        "message": MESSAGE
    });
    Json(json_response)
}*/

#[tokio::main]
async fn main() {
    /*let question = Question::new(
        "1".to_string(),
        "First Question".to_string(),
        "Contents of question".to_string(),
        Some(vec!("faq".to_string())),
    );*/
    
    let app = Router::new().route("/api/questions", get(api::get_questions));
    println!("Server started successfully");
    let listener = tokio::net::TcpListener::bind("127.0.0.1:8000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}