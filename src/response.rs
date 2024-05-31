use crate::model::Question;
use serde::Serialize;

#[derive(Debug, Deserialize, Serialize)]
pub struct QuestionResponse {
    pub id: String,
    pub title: String,
    pub content: String,
    //tags
}

/*#[derive(Serialize)]
pub struct GenericResponse {
    pub status: String,
    pub message: String,
}

#[derive(Serialize, Debug)]
pub struct QuestionData {
    pub question: Question,
}

#[derive(Serialize, Debug)]
pub struct OneQuestionResponse {
    pub status: String,
    pub data: QuestionData,
}

#[derive(Serialize, Debug)]
pub struct QuestionListResponse {
    pub status: String,
    pub results: usize,
    pub questions: Vec<Question>,
}*/