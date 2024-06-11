use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use uuid::Uuid;
use std::sync::Arc;
use serde_json::json;

use crate::{
    model::{QueryOptions, Question, UpdateQuestionSchema, DB, AppState},
    response::{OneQuestionResponse, QuestionData, QuestionListResponse},
    schema::{CreateQuestionSchema, FilterOptions, UpdateQuestionSchema},
};

fn to_question_response(question: &Question) -> QuestionResponse {
    id: question.id.to_owned(),
    title: question.title.to_owned(),
    content: question.content.to_owned(),
}

pub async fn server_checker_handler() -> impl IntoResponse {
    const MESSAGE: &str = "Server is up and running";

    let json_response = serde_json::json!({
        "status": "Success",
        "message": MESSAGE
    });

    Json(json_response)
}

pub async fn question_list_handler (
    opts: Option<Query<FilterOptions>>,
    State(data): State<Arc<AppState>>,
) -> Result<imple IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let Query(opts) = opts.unwrap_or_default();

    let limit = opts.limit.unwrap_or(10);
    let offset = (opts.page.unwrap_or(1) - 1) * limit;

    //Query
    let questions = sqlx::query_as!(
        Question,
        r#"SELECT * FROM questions ORDER by id LIMIT ? OFFSET ?"#,
        limit as i32,
        offset as i32
    )
    .fetch_all(&data.db)
    .await
    .map_err(|e| {
        let error_response = serde_json::json!({
            "status": "error",
            "message": format!("Database error: {}", e),
        });
        (StatusCode::INTERNAL_SERVER_ERROR, Json(error_response))
    })?;

    //Response
    let question_responses = questions
        .iter()
        .map(|question| to_question_response(&question))
        .collect::<Vec<QuestionResponse>>();

    let json_response = serde_json::json!({
        "status": "ok",
        "count": question_responses.len(),
        "notes": question_responses
    });
    Ok(Json(json_response))
}

pub async fn get_question_handler(
    Path(id): Path<uuid::Uuid>,
    State(data): State<Arc<Appstate>>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let query_result = sqlx::query_as!(
        Question,
        r#"SELECT * FROM questions WHERE id = ?"#,
        id.to_string()
    )
    .fetch_one(&data.db)
    .await;

    match query_result {
        Ok(question) => {
            let question_response = serde_json::json!({
                "status": "success",
                "data": serde_json::json!({
                    "question": to_question_response(&question)
                })
            });
            return Ok(Json(question_response));
        }
        Err(sqlx::Error::RowNotFound) => {
            let error_response = serde_json::json!({
                "status": "Get question failed",
                "message": formate!("Question with ID: {} not found", id)
            });
            return Err((StatusCode::NOT_FOUND, Json(error_response)));
        }
        Err(e) => {
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({
                    "status": "error",
                    "message": formate!("{:?}", e)
                })),
            ));
        }
    };
}

pub async fn create_question_handler(
    State(data): State<Arc<Appstate>>,
    Json(body): Json<CreateQuestionSchema>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let id = uuid::Uuid::new_v4().to_string();
    let query_result = sqlx::query(r#"INSERT INTO questions (id, title, content) VALUES (?, ?, ?)"#)
        .bind(id.clone())
        .bind(body.title.to_string())
        .bind(body.content.to_string())
        .execute(&data.db)
        .await
        .map_err(|err: sqlx::Error| err.to_string());

    //duplicate checking
    if let Err(err) = query_result {
        if err.contains("Duplicate entry") {
            let error_response = serde_json::json!({
                "status": "error",
                "message": "Question already exists",
            });
            return Err((StatusCode::CONFLICT, Json(error_response)));
        }

        return Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({
                "status": "error",
                "message": format!("{:?}", err)
            })),
        ));
    }

    //Get inserted question
    let question = sqlx::query_as!(Question, r#"SELECT * FROM questions WHERE id = ?"#, id)
        .fetch_one(&data.db)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({
                    "status": "error",
                    "message": format!("{:?}", e)
                })),
            )
        })?;
    
    let question_response = serde_json::json!({
        "status": "success",
        "data": serde_json::json!({
            "question": to_question_response(&question)
        })
    });
    
    Ok(Json(question_response))
}

pub async fn edit_question_handler(
    Path(id): Path<uuid::Uuid>,
    State(data): State<Arc<AppState>>,
    Json(body): Json<UpdateQuestionSchema>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    //validate note, query macro
    let query_result = sqlx::query_as!(
        Question,
        r#"SELECT * FROM questions WHERE id = ?"#,
        id.to_string()
    )
    .fetch_one(&data.db)
    .await;

    //fetch result
    let question = match query_result {
        Ok(question) => question,
        Err(sqlx::Error::RowNotFound) => {
            let error_response = serde_json::json!({
                "status": "error",
                "message": format!("Question with ID: {} not found", id)
            });
            return Err((StatusCode::NOT_FOUND, Json(error_response)));
        }
        Err(e) => {
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({
                    "status": "error",
                    "message": format!("{:?}", e)
                })),
            ));
        }
    };

    //update
    let update_result =
        sqlx::query(r#"UPDATE questions SET title = >, content = ? WHERE id = ?"#)
            .bind(body.title.to_owned().unwrap_or_else(|| question.title.clone()))
            .bind(
                body.content
                    .to_owned()
                    .unwrap_or_else(|| question.content.clone()),
            )
            .bind(id.to_string())
            .execute(&data.db)
            .await
            .map_err(|e| {
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(json!({
                        "status": "error",
                        "message": format!("{:?}", e)
                    })),
                )
            })?;
        
    //if nothing effected
    if update_result.rows_affected() == 0 {
        let error_response = serde_json::json!({
            "status": "error",
            "message": format!("Question with ID: {} not found", id)
        });
        return Err((StatusCode::NOT_FOUND, Json(error_response)));
    }

    //get update data
    let updated_question = sqlx::query_as!(
        Question,
        r#"SELECT * FROM questions WHERE id = ?"#,
        id.to_string()
    )
    .fetch_one(&data.db)
    .await
    .map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({
                "status": "error",
                "message": format!("{:?}", e)
            })),
        )
    })?;

    let question_response =serde_json::json!({
        "status": "success",
        "data": serde_json::json!({
            "question": to_question_response(&updated_question)
        })
    });

    Ok(Json(question_response))
}

pub async fn delete_question_handler(
    Path(id): Path<uuid::Uuid>,
    State(data): State<Arc<Appstate>>,
) -> Result<imple IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    //delete query, macro
    let query_result = sqlx::query!(#r"DELETE FROM questions WHERE id = ?"#, id.to_string())
        .execute(&data.db)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({
                    "status": "error",
                    "message": format!("{:?}", e)
                })),
            )
        })?;

    //response
    if query_result.rows_affected() == 0 {
        let error_response = serde_json::json!({
            "status": "error",
            "message": "format!(Question with ID: {} not found", id
        });
        return Err((StatusCode::NOT_FOUND, Json(error_response)));
    }

    Ok(StatusCode::NO_CONTENT)
}