use serde::{Deserialize, Serialize};
use warp::filters::cors::CorsForbidden;
use warp::http::Method;
use warp::http::StatusCode;
use warp::reject::Reject;
use warp::Filter;
use warp::Rejection;
use warp::Reply;

use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

/* #[derive(Debug)]
struct InvalidId;
impl Reject for InvalidId {} */

/// Represents a valid identifier (ID) for a question.
#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq, Hash)]
struct QuestionId(String);

impl std::fmt::Display for QuestionId {
  fn fmt(
    &self,
    f: &mut std::fmt::Formatter<'_>,
  ) -> std::fmt::Result {
    write!(f, "id: {}", self.0)
  }
}

impl std::str::FromStr for QuestionId {
  type Err = std::io::Error;

  fn from_str(id: &str) -> Result<Self, Self::Err> {
    match id.is_empty() {
      false => Ok(QuestionId(id.to_string())),
      true => Err(std::io::Error::new(
        std::io::ErrorKind::InvalidInput,
        "No ID provided",
      )),
    }
  }
}

/// Represents a question posted in the system.
#[derive(Debug, Clone, Serialize, Deserialize)]
struct Question {
  /// Identifier of the question.
  id: QuestionId,
  /// Title of the question.
  title: String,
  /// Text contents of the question.
  content: String,
  /// List of tags for the question.
  tags: Option<Vec<String>>,
}

impl std::fmt::Display for Question {
  fn fmt(
    &self,
    f: &mut std::fmt::Formatter<'_>,
  ) -> std::fmt::Result {
    write!(
      f,
      "{}, title: {}, content: {}, tags: {:?}",
      self.id, self.title, self.content, self.tags
    )
  }
}

/// Get a set of questions from the given parameters and data store.
///
/// # Arguments
///
/// * `params`: Parameters for refining the set of questions to retrieve.
/// * `store`: Data store that contains all the questions.
async fn get_questions(
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
    // Retrieve the result set as a slice of elements between the start and end indexes.
    let result_set: &[Question] = &data[pagination.start..pagination.end];
    Ok(warp::reply::json(&result_set))
  } else {
    let data: Vec<Question> = store.questions.read().await.values().cloned().collect();
    Ok(warp::reply::json(&data))
  }
}

async fn add_question(
  store: Store,
  question: Question,
) -> Result<impl warp::Reply, warp::Rejection> {
  store
    .questions
    .write()
    .await
    .insert(question.id.clone(), question);

  Ok(warp::reply::with_status("Question added", StatusCode::OK))
}

/// Represents the start and end index of a set of results.
#[derive(Debug)]
struct Pagination {
  /// Start index of a set of results.
  start: usize,
  /// End index of a set of results.
  end: usize,
}

/// Gets a pagination object from the given set of parameters.
///
/// Swaps the start and end indexes if the start index is greater than the end index.
///
/// # Arguments
///
/// * `params`: Parameters for refining the set of results to retrieve.
///
fn extract_pagination(params: HashMap<String, String>) -> Result<Pagination, Error> {
  if params.contains_key("start") && params.contains_key("end") {
    let start_index: usize = params
      .get("start")
      .unwrap()
      .parse::<usize>()
      .map_err(Error::ParseError)?;
    let end_index: usize = params
      .get("end")
      .unwrap()
      .parse::<usize>()
      .map_err(Error::ParseError)?;

    // Swap start and end indexes if the start index is greater than the end index
    let (start_index, end_index) = if start_index > end_index {
      (end_index, start_index)
    } else {
      (start_index, end_index)
    };

    let pagination = Pagination {
      start: start_index,
      end: end_index,
    };

    return Ok(pagination);
  }

  Err(Error::MissingParameters)
}

/// Represents an error for processing query parameters.
#[derive(Debug)]
enum Error {
  /// An kind of error for parsing errors.
  ParseError(std::num::ParseIntError),
  /// A kind of error for missing parameters.
  MissingParameters,
}

impl std::fmt::Display for Error {
  fn fmt(
    &self,
    f: &mut std::fmt::Formatter<'_>,
  ) -> std::fmt::Result {
    match *self {
      Error::ParseError(ref err) => {
        write!(f, "Cannot parse the parameter: {}", err)
      }
      Error::MissingParameters => write!(f, "Missing parameter"),
    }
  }
}

impl Reject for Error {}

/// Returns a Warp error reply for the given rejection.
///
/// # Arguments
///
/// * `rej`: Warp rejection object containing an error that happened.
async fn return_error(rej: Rejection) -> Result<impl Reply, Rejection> {
  // Handle operations errors
  if let Some(error) = rej.find::<Error>() {
    Ok(warp::reply::with_status(
      error.to_string(),
      StatusCode::RANGE_NOT_SATISFIABLE,
    ))
  }
  // Handle CORS errors
  else if let Some(error) = rej.find::<CorsForbidden>() {
    Ok(warp::reply::with_status(
      error.to_string(),
      StatusCode::FORBIDDEN,
    ))
  }
  // At this point, the possible rejection is that a path not found
  else {
    Ok(warp::reply::with_status(
      "Route not found".to_string(),
      StatusCode::NOT_FOUND,
    ))
  }
}

/// Represents the data store for the application.
#[derive(Clone)]
struct Store {
  /// Collection of questions in the data store.
  questions: Arc<RwLock<HashMap<QuestionId, Question>>>,
}

impl Store {
  /// Creates a new data store.
  fn new() -> Self {
    Self {
      questions: Arc::new(RwLock::new(Self::init())),
    }
  }

  /// Initializes the data store with available data.
  fn init() -> HashMap<QuestionId, Question> {
    let file = include_str!("../questions.json");
    serde_json::from_str(file).expect("cannot read the questions.json file.")
  }
}

#[tokio::main]
async fn main() {
  let store = Store::new();
  let store_filter = warp::any().map(move || store.clone());

  let cors = warp::cors()
    .allow_any_origin()
    .allow_header("content-type")
    .allow_methods(&[Method::PUT, Method::DELETE, Method::GET, Method::POST]);

  let get_questions = warp::get()
    .and(warp::path("questions"))
    .and(warp::path::end())
    .and(warp::query())
    .and(store_filter.clone())
    .and_then(get_questions);

  let add_question = warp::post()
    .and(warp::path("questions"))
    .and(warp::path::end())
    .and(store_filter.clone())
    .and(warp::body::json())
    .and_then(add_question);

  let routes = get_questions
    .or(add_question)
    .with(cors)
    .recover(return_error);

  warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}
