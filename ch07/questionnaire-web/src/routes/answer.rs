use warp::hyper::StatusCode;

use crate::{store::Store, types::answer::NewAnswer};

/// Adds a new answer with the given parameters to a data store.
///
/// # Arguments
///
/// * `store`: Data store for where answer will be saved.
/// * `answer`: New answer to be added to the datastore.
pub async fn add_answer(
  store: Store,
  answer: NewAnswer,
) -> Result<impl warp::Reply, warp::Rejection> {
  match store.add_answer(answer).await {
    Ok(_) => Ok(warp::reply::with_status(
      "Answer added",
      StatusCode::CREATED,
    )),
    Err(err) => Err(warp::reject::custom(err)),
  }
} // end fn add_answer()
