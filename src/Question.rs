pub struct Question {
    id: QuestionId,
    title: String,
    content: String,
    tags: Option<Vec<String>>,
}

struct QuestionId(String);

impl Question {
    pub fn new(
        id: QuestionId,
        title: String,
        content: String,
        tags: Option<Vec<String>>
    ) -> Self {
        Question {
            id, title, content, tags,
        }
    }
}