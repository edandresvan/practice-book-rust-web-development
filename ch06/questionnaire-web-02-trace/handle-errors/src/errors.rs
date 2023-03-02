use warp::filters::body::BodyDeserializeError;
use warp::filters::cors::CorsForbidden;
use warp::hyper::StatusCode;
use warp::reject::Reject;
use warp::{Rejection, Reply};

/// Represents an error for processing query parameters.
#[derive(Debug)]
pub enum QError {
  /// An kind of error for parsing errors.
  ParseError(std::num::ParseIntError),
  /// A kind of error for missing parameters.
  MissingParameters,
  /// A kind of error for questions not found.
  QuestionNotFound,
} // end enum QError

impl std::fmt::Display for QError {
  fn fmt(
    &self,
    f: &mut std::fmt::Formatter<'_>,
  ) -> std::fmt::Result {
    match *self {
      QError::ParseError(ref err) => {
        write!(f, "Cannot parse the parameter: {}", err)
      }
      QError::MissingParameters => write!(f, "Missing parameter."),
      QError::QuestionNotFound => write!(f, "Question not found."),
    }
  }
}

impl Reject for QError {}

/// Returns a Warp error reply for the given rejection.
///
/// # Arguments
///
/// * `rej`: Warp rejection object containing an error that happened.
pub async fn return_error(rej: Rejection) -> Result<impl Reply, Rejection> {
  // Handle operations errors
  if let Some(error) = rej.find::<QError>() {
    match error {
      QError::QuestionNotFound => Ok(warp::reply::with_status(
        error.to_string(),
        StatusCode::NOT_FOUND,
      )),
      QError::MissingParameters => Ok(warp::reply::with_status(
        error.to_string(),
        StatusCode::BAD_REQUEST,
      )),
      QError::ParseError(_) => Ok(warp::reply::with_status(
        error.to_string(),
        StatusCode::BAD_REQUEST,
      )),
      // _ => Ok(warp::reply::with_status(
      //   error.to_string(),
      //   StatusCode::RANGE_NOT_SATISFIABLE,
      // )),
      // _ => Ok(warp::reply::with_status(
      //   error.to_string(),
      //   StatusCode::NOT_FOUND,
      // )),
    }
  }
  // Handle CORS errors
  else if let Some(error) = rej.find::<CorsForbidden>() {
    Ok(warp::reply::with_status(
      error.to_string(),
      StatusCode::FORBIDDEN,
    ))
  }
  // Handle malformed HTTP Bodies
  else if let Some(error) = rej.find::<BodyDeserializeError>() {
    Ok(warp::reply::with_status(
      error.to_string(),
      StatusCode::UNPROCESSABLE_ENTITY,
    ))
  }
  // At this point, the possible rejection is that a path not found
  else {
    Ok(warp::reply::with_status(
      "Route not found".to_string(),
      StatusCode::NOT_FOUND,
    ))
  }
} // end fn return_error()
