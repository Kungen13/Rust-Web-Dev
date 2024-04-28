use crate::*;
use axum::response::*;
use axum::extract::*;
use std::sync::*;

pub async fn get_questions(/*State(Question): State<Arc<RwLock<Question>>>*/) -> Response {
    let question = Question::new(
        "1".to_string(),
        "Color".to_string(),
        "What is your favorite color".to_string(),
        Some(vec!("color_q".to_string())),
    );
    return question;
}