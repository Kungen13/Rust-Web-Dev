use axum::{
    routing::{get, post},
    Router,
};

use crate::{
    handler::{
        server_checker_handler,
        create_question_handler, delete_question_handler,
        update_question_handler, get_question_handler,
        question_list_handler,
    },
    model,
};

pub fn create_router() -> Router {
    let db = model::question_db();

    Router::new()
    .route("/api/serverchecker", get(server_checker_handler))
    .route("/api/questions", post(create_question_handler).get(question_list_handler),)
    .route("/api/questions/:id", get(get_question_handler).patch(update_question_handler).delete(delete_question_handler))
    .with_state(db)
}