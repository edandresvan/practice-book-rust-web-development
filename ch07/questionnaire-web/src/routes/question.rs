use std::collections::HashMap;
use tracing::{event, instrument, Level};
use warp::hyper::StatusCode;

use handle_errors::errors::QError;

use crate::{
  store::Store,
  types::{
    pagination::{extract_pagination, Pagination},
    question::{NewQuestion, Question},
  },
};

/// Gets a set of questions from the given parameters and data store.
///
/// # Arguments
///
/// * `params`: Parameters to filter the set of questions to retrieve.
/// * `store`: Data store that contains all the questions.
#[instrument]
pub async fn get_questions(
  params: HashMap<String, String>,
  store: Store,
) -> Result<impl warp::Reply, warp::Rejection> {
  event!(target: "questionnaire web api", Level::INFO, "querying questions");
  // Default pagination
  let mut pagination: Pagination = Pagination::default();

  //
  if !params.is_empty() {
    event!(Level::INFO, pagination = true);
    // Create the pagination object from the given HTTP parameters.
    pagination = extract_pagination(params)?;
  } else {
    event!(Level::INFO, pagination = false);
  }

  match store
    .get_questions(pagination.offset, pagination.limit)
    .await
  {
    Ok(questions) => Ok(warp::reply::json(&questions)),
    Err(err) => Err(warp::reject::custom(err)),
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
  question: NewQuestion,
) -> Result<impl warp::Reply, warp::Rejection> {
  match store.add_question(question).await {
    Ok(questions) => Ok(warp::reply::with_status(
      warp::reply::json(&questions),
      StatusCode::CREATED,
    )),
    Err(err) => Err(warp::reject::custom(QError::DatabaseQueryError(err))),
  }
} // end fn add_question()

/// Updates an existing question with the given the ID and data store.
///
/// # Arguments
///
/// * `id`: ID (unique identifier) of the question to be updated.
/// * `store`: Data store that contains all the questions.
/// * `question`: Question to add to the data store.
pub async fn update_question(
  id: i32,
  store: Store,
  question: Question,
) -> Result<impl warp::Reply, warp::Rejection> {
  match store.update_question(question, id).await {
    Ok(questions) => Ok(warp::reply::with_status(
      warp::reply::json(&questions),
      StatusCode::OK,
    )),
    Err(err) => Err(warp::reject::custom(QError::DatabaseQueryError(err))),
  }
} // end fn update_question()

/// Deletes an existing question with the given the ID and data store.
///
/// # Arguments
///
/// * `id`: ID (unique identifier) of the question to be deleted.
/// * `store`: Data store that contains all the questions.
pub async fn delete_question(
  id: i32,
  store: Store,
) -> Result<impl warp::Reply, warp::Rejection> {
  match store.delete_question(id).await {
    Ok(1..=u64::MAX) => Ok(warp::reply::with_status(
      format!("Question {} deleted.", id),
      StatusCode::OK,
    )),
    Ok(0) => Err(warp::reject::custom(QError::QuestionNotFound)),

    Err(err) => Err(warp::reject::custom(QError::DatabaseQueryError(err))),
  }
} // fn delete_question()
