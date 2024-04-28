//use std::io::{Error, ErrorKind};
//use std::str::FromStr;

#[derive(Debug)]
pub struct Question {
    id: String,
    title: String,
    content: String,
    tags: Option<Vec<String>>,
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

/*impl std::fmt::Debug for Question {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{:?}", self.tags)
    }
}*/