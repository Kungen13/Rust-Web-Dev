use crate::*;

#[derive(Serialize)]
pub struct GenericResponse {
    pub status: String,
    pub message: String,
}

#[derive(Serialize, Debug)]
pub struct QuestionData {
    pub question: Question,
}

#[derive(Serialize, Debug)]
pub struct SingleQuestionResponse {
    pub status: String,
    pub data: QuestionData,
}

#[derive(Serialize, Debug)]
pub struct QuestionListResponse {
    pub status: String,
    pub data: HashMap<String, Question>,
}

