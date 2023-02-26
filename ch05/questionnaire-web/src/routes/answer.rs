use std::collections::HashMap;

use warp::hyper::StatusCode;

use crate::{
  store::Store,
  types::{
    answer::{Answer, AnswerId},
    question::QuestionId,
  },
};

/// Adds a new answer with the given parameters to a data store.
///
/// # Arguments
///
/// * `store`: Data store for where answer will be saved.
/// * `params`: Set of parameters with data for adding a new answer.
pub async fn add_answer(
  store: Store,
  params: HashMap<String, String>,
) -> Result<impl warp::Reply, warp::Rejection> {
  let answer = Answer {
    id: AnswerId("1".to_string()),
    content: params.get("content").unwrap().to_string(),
    question_id: QuestionId(params.get("question_id").unwrap().to_string()),
  };

  store
    .answers
    .write()
    .await
    .insert(answer.id.clone(), answer);

  Ok(warp::reply::with_status("Answer added", StatusCode::OK))
} // end fn add_answer()
