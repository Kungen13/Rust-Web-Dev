#![allow(warnings)]

mod question_f;
mod api;
mod response;

use question_f::*;
use api::*;
use response::*;

use std::collections::HashMap;
use std::sync::Arc;
use tokio::{self, sync::RwLock};
use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{delete, get, post, put},
    Json, Router,
};
use serde::{ser::SerializeStruct, Deserialize, Serialize, Serializer};


/*async fn health_checker_handler() -> impl IntoResponse {
    //(StatusCode::NOT_FOUND, "404 Not Found").into_response()
    const MESSAGE: &str = "Build Simple CRUD API in Rust using Axum";
    let json_response = serde_json::json!({
        "status": "success",
        "message": MESSAGE
    });
    Json(json_response)
}*/

/*async fn handler_404() -> Response {
    (StatusCode::NOT_FOUND, "404 Not Found").into_response()
}*/

#[tokio::main]
async fn main() {
    let store = Store::new();

    let app = Router::new()
    .route("/api/questions", get(api::get_questions(store.questions, store, "1".to_string())));

    println!("Server started successfully");

    let listener = tokio::net::TcpListener::bind("127.0.0.1:8000").await.unwrap();
    axum::serve(listener, app).await.unwrap();

}