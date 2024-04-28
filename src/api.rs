use crate::*;

pub async fn get_questions() -> Response {
    let question = Question::new(
        "1".to_string(),
        "First Question".to_string(),
        "Contents of question".to_string(),
        Some(vec!("faq".to_string())),
    );
}