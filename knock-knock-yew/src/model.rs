use crate::*;

/*pub struct AppState {
    db: MySqlPool,
}*/

#[derive(Properties, Clone, PartialEq, serde::Deserialize)] //removed clone
pub struct Question {
    pub id: String,
    pub title: String,
    pub content: String,
}

impl Question {
    pub async fn get_question(key: Option<String>) -> Msg {
        let request = match &key {
            None => "http://localhost:3000/api/v1/joke".to_string(),
            Some(ref key) => format!("http://localhost:3000/api/v1/joke/{}", key,),
        };
        let response = http::Request::get(&request).send().await;
        match response {
            Err(e) => Msg::GotQuestion(Err(e)),
            Ok(data) => Msg::GotQuestion(data.json().await),
        }
    }
}

#[derive(Properties, Clone, PartialEq, serde::Deserialize)]
pub struct QuestionProps {
    pub question: Question,
}

#[function_component(QuestionComp)]
pub fn question(question: &QuestionProps) -> Html {
    let question = &question.question;
    html! { <>
        <div class="question">
            <span class="teller">{question.title.clone()}</span><br/>
            <span class="teller">{question.content.clone()}</span>
        </div>
        <span class="annotation">
            {format!("[id: {}", &question.id)}
            {"]"}
        </span>
    </> }
}

/*#[derive(Debug, Deserialize, Default)]
pub struct QueryOptions {
    pub page: Option<usize>,
    pub limit: Option<usize>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct UpdateQuestionSchema {
    pub title: Option<String>,
    pub content: Option<String>,
}*/