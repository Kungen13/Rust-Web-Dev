use serde::{Deserialize, Serialize};

//List
pub struct FilterOptions {
    pub page: Option<usize>,
    pub limit: Option<usize>,
}

//Read/Delete
#[derive(Deserialize, Debug)]
pub struct ParamOptions {
    pub id: String,
}

//Create
#[derive(Serialize, Deserialize, Debug)]
oub struct CreateQuestionSchema {
    pub title: String,
    pub content: String,
}

//Update
#[derive(Serialize, Deserialize, Debug)]
pub struct UpdateQuestionSchema {
    pub title: Option<String>,
    pub content: Option<String>,
}