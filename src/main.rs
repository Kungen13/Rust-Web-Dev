#[derive(Debug)]
struct Question {
    id: QuestionId,
    title: String,
    content: String,
    tags: Option<Vec<String>>,
}

#[derive(Debug)]
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

impl std::fmt::Display for Question {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}. title: {}, content: {}, tags{:?}",
            self.id, self.title, self.content, self.tags
        )
    }
}

impl std::fmt::Display for QuestionId {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        write!(f, "id: {}", self.0)
    }
}

impl std::fmt::Debug for Question {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{:?}", self.tags)
    }
}

fn main() {
    let question = Question::new(
        QuestionId("1".to_string()),
        "First Question".to_string(),
        "Contents of question".to_string(),
        Some(vec!("faq".to_string())),
    );
    println!("{:?}", question);
}
//page 62 stop
