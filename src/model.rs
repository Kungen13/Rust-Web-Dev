use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::Mutex;
use crate::MySqlPool;

pub struct AppState {
    db: MySqlPool,
}

#[derive(Debug, Deserialize, Serialize, sqlx::FromRow)] //removed clone
pub struct Question {
    pub id: String,
    pub title: String,
    pub content: String,
    //pub tags: Option<Vec<String>>,
}

pub type DB = Arc<Mutex<Vec<Question>>>;

pub fn question_db() -> DB {
    Arc::new(Mutex::new(Vec::new()))
}

#[derive(Debug, Deserialize, Default)]
pub struct QueryOptions {
    pub page: Option<usize>,
    pub limit: Option<usize>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct UpdateQuestionSchema {
    pub title: Option<String>,
    pub content: Option<String>,
}