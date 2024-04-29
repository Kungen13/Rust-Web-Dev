use crate::*;
use crate::*;

#[derive(Debug, Serialize, Deserialize)]
pub struct Question {
    pub id: String,
    pub title: String,
    pub content: String,
    pub tags: Option<Vec<String>>,
}

impl Question {
    pub fn new(
        id: String,
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
            "{},
            title: {},
            content: {},
            tags{:?}",
            self.id, self.title, self.content, self.tags
        )
    }
}

//#[derive(Clone)] ---- page 107
pub struct Store {
    pub questions: Arc<RwLock<HashMap<String, Question>>>,
}
//page 102 if getting errors for id
impl Store {
    pub fn new() -> Self{
        Store {
            questions: Arc::new(RwLock::new(Self::init())),
        }
    }

    pub fn init(self) -> Self {
        let question = Question::new(
            "1".to_string(),
            "How?".to_string(), 
            "Please help!".to_string(), 
            Some(vec!["general".to_string()])
        );
        self.add_question(question)
    }

    pub fn add_question(mut self, question: Question) -> Self {
        self.questions.insert(question.id.clone(), question);
        self
    }
}

/*impl std::fmt::Debug for Question {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{:?}", self.tags)
    }
}*/