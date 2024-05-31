use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use uuid::Uuid;

use crate::{
    model::{QueryOptions, Question, UpdateQuestionSchema, DB},
    response::{OneQuestionResponse, QuestionData, QuestionListResponse},
};

pub async fn server_checker_handler() -> impl IntoResponse {
    const MESSAGE: &str = "Server is up and running";

    let json_response = serde_json::json!({
        "status": "Success",
        "message": MESSAGE
    });

    Json(json_response)
}

pub async fn question_list_handler(
    options: Option<Query<QueryOptions>>,
    State(db): State<DB>,
) -> impl IntoResponse {
    let questions = db.lock().await;

    let Query(options) = options.unwrap_or_default();

    let limit = options.limit.unwrap_or(10);
    let offset = (options.page.unwrap_or(1) - 1) * limit;

    let questions: Vec<Question> = questions.clone().into_iter().skip(offset).take(limit).collect();

    let json_response = QuestionListResponse {
        status: "success".to_string(),
        results: questions.len(),
        questions,
    };

    Json(json_response)
}

pub async fn create_question_handler(
    State(db): State<DB>,
    Json(mut body): Json<Question>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let mut vec = db.lock().await;

    if let Some(question) = vec.iter().find(|question| question.title == body.title) {
        let error_response = serde_json::json!({
            "status": "fail",
            "message": format!("Question with title: '{}' already exists", question.title),
        });
        return Err((StatusCode::CONFLICT, Json(error_response)));
    }

    let uuid_id = Uuid::new_v4();

    body.id = Some(uuid_id.to_string());
    //body.tags
    
    let question = body.to_owned();

    vec.push(body);

    let json_response = OneQuestionResponse {
        status: "success".to_string(),
        data: QuestionData { question },
    };

    Ok((StatusCode::CREATED, Json(json_response)))
}

pub async fn get_question_handler(
    Path(id): Path<Uuid>,
    State(db): State<DB>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let id = id.to_string();
    let vec = db.lock().await;

    if let Some(question) = vec.iter().find(|question| question.id == Some(id.to_owned())) {
        let json_response = OneQuestionResponse {
            status: "success".to_string(),
            data: QuestionData { question: question.clone() },
        };
        return Ok((StatusCode::OK, Json(json_response)));
    }

    let error_response = serde_json::json!({
        "status": "fail",
        "message": format!("Question with ID: {} not found", id)
    });
    Err((StatusCode::NOT_FOUND, Json(error_response)))
}

pub async fn update_question_handler(
    Path(id): Path<Uuid>,
    State(db): State<DB>,
    Json(body): Json<UpdateQuestionSchema>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let id = id.to_string();
    let mut vec = db.lock().await;

    if let Some(question) = vec.iter_mut().find(|question| question.id == Some(id.clone())) {
        let title = body.title.to_owned().unwrap_or_else(|| question.title.to_owned());
        let content = body.content.to_owned().unwrap_or_else(|| question.content.to_owned());

        let payload = Question {
            id: question.id.to_owned(),
            title: if !title.is_empty() {
                title
            } else {
                question.title.to_owned()
            },
            content: if !content.is_empty() {
                content
            } else {
                question.content.to_owned()
            },
            //tags: 
        };
        *question = payload;

        let json_response = OneQuestionResponse {
            status: "success".to_string(),
            data: QuestionData { question: question.clone() },
        };
        Ok((StatusCode::OK, Json(json_response)))
    } else {
        let error_response = serde_json::json!({
            "status": "fail",
            "message": format!("Question with ID: {} not found", id)
        });
        Err((StatusCode::NOT_FOUND, Json(error_response)))
    }
}

pub async fn delete_question_handler(
    Path(id): Path<Uuid>,
    State(db): State<DB>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let id = id.to_string();
    let mut vec = db.lock().await;

    if let Some(pos) = vec.iter().position(|question| question.id == Some(id.clone())) {
        vec.remove(pos);
        return Ok((StatusCode::NO_CONTENT, Json("")));
    }

    let error_response = serde_json::json!({
        "status": "fail",
        "message": format!("Question with ID: {} not found", id)
    });
    Err((StatusCode::NOT_FOUND, Json(error_response)))
}