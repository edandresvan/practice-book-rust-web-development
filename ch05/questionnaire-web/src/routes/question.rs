use std::collections::HashMap;
use warp::hyper::StatusCode;

use handle_errors::errors::QError;

use crate::{
  store::Store,
  types::{
    pagination::{extract_pagination, Pagination},
    question::{Question, QuestionId},
  },
};

/// Gets a set of questions from the given parameters and data store.
///
/// # Arguments
///
/// * `params`: Parameters to filter the set of questions to retrieve.
/// * `store`: Data store that contains all the questions.
pub async fn get_questions(
  params: HashMap<String, String>,
  store: Store,
) -> Result<impl warp::Reply, warp::Rejection> {
  if !params.is_empty() {
    let mut pagination: Pagination = extract_pagination(params)?;
    // Clone each question because collect() requires owernship of each question value.
    let data: Vec<Question> = store.questions.read().await.values().cloned().collect();
    // Check a valid range of results
    if pagination.end > data.len() {
      pagination.end = data.len();
    }
    if pagination.start < 1 {
      pagination.start = 1;
    }
    // Retrieve the result set as a slice of elements between the start and end indexes.
    let result_set: &[Question] = &data[(pagination.start - 1)..pagination.end];
    Ok(warp::reply::json(&result_set))
  } else {
    let data: Vec<Question> = store.questions.read().await.values().cloned().collect();
    Ok(warp::reply::json(&data))
  }
}

/// Adds a new question to the given data store.
///
/// # Arguments
///
/// * `store`: Data store that contains all the questions.
/// * `question`: Question to add to the data store.
pub async fn add_question(
  store: Store,
  question: Question,
) -> Result<impl warp::Reply, warp::Rejection> {
  store
    .questions
    .write()
    .await
    .insert(question.id.clone(), question);

  Ok(warp::reply::with_status("Question added", StatusCode::OK))
} // end fn add_question()

/// Updates an existing question with the given the ID and data store.
///
/// # Arguments
///
/// * `id`: ID (unique identifier) of the question to be updated.
/// * `store`: Data store that contains all the questions.
/// * `question`: Question to add to the data store.
pub async fn update_question(
  id: String,
  store: Store,
  question: Question,
) -> Result<impl warp::Reply, warp::Rejection> {
  match store.questions.write().await.get_mut(&QuestionId(id)) {
    Some(q) => {
      *q = question;
      Ok(warp::reply::with_status("Question updated", StatusCode::OK))
    }
    None => Err(warp::reject::custom(QError::QuestionNotFound)),
  }
} // end fn update_question()

/// Deletes an existing question with the given the ID and data store.
///
/// # Arguments
///
/// * `id`: ID (unique identifier) of the question to be deleted.
/// * `store`: Data store that contains all the questions.
pub async fn delete_question(
  id: String,
  store: Store,
) -> Result<impl warp::Reply, warp::Rejection> {
  match store.questions.write().await.remove(&QuestionId(id)) {
    Some(_) => Ok(warp::reply::with_status(
      "Question deleted.",
      StatusCode::OK,
    )),
    None => Err(warp::reject::custom(QError::QuestionNotFound)),
  }
} // fn delete_question()
