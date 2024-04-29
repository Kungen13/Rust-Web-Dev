use crate::*;

pub async fn get_questions(
    params: HashMap<String, Question>,
    store: Store,
    key: String
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    
    /*if params.contains_key(&key)
    {
        let json_response = QuestionListResponse {
            status: "get_questions success".to_string(),
            data: params,
        };
        return Ok((StatusCode::OK, Json(json_response)))
    }

    let error_response = serde_json::json!({
        "status": "get_question failed",
        "message": format!{"Question with ID: {} not found", key}
    });
    Err((StatusCode::NOT_FOUND, Json(error_response)))*/
}

pub asyn fn 


//These are implementations found at https://codevoweb.com/create-a-simple-api-in-rust-using-the-axum-framework/#define-the-api-response-structs
//as a reference for axum.

/*pub async fn create_question(
    State(question): State<Question>,
    Json(mut body): Json<Question>,
) -> impl Response<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    
    /*if body.title.is_empty()
    {
        let error_response = serde_json::json!({
            "status": "create error",
            "message": "create_question was not passed a title, could not create question",
        });
        return Err((StatusCode::CONFLICT, Json(error_response)));
    }*/
    //temporary question generation
    let new_id: u8 = rng.gen();

    body.id = Some(new_id.to_string());
    body.title = Some("new-title".to_string());
    body.contents = Some("new-contents".to_string());
    body.tags.push("new-tag".to_string());

    let question = body.to_owned();
    
    let json_response = SingleQuestionResponse {
        status: "success".to_string(),
        data: QuestionData {question},
    };

    Ok((StatusCode::CREATED, Json(json_response)))

}*/

/*pub async fn get_question(
    Path(id): Path<String>,
    State<question>: State<Question>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {

    if question.id == id
    {
        let json_response = SingleQuestionResponse {
            status: "success".to_string(),
            data: QuestionData {Question: question.clone()},
        };
        return Ok((StatusCode::OK, Json(json_response)));
    }

    let error_response = serde_json::json!({
        "status": "get_question failed",
        "message": format!{"Question with ID: {} not found". id}
    });
    Err((StatusCode::NOT FOUND, Json(error_response)))
}

pub async fn update_question(
    Path(id): Path<String>,
    State(question): State<Question>,
    Json(body): Json<UpdateQuestionSchema>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {

}*/