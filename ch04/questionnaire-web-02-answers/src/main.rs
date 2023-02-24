use serde::{Deserialize, Serialize};
use warp::filters::body::BodyDeserializeError;
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

/// Gets a set of questions from the given parameters and data store.
///
/// # Arguments
///
/// * `params`: Parameters to filter the set of questions to retrieve.
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
    if pagination.start < 1  {
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
} // end fn add_question()

/// Updates an existing question with the given the ID and data store.
///
/// # Arguments
///
/// * `id`: ID (unique identifier) of the question to be updated.
/// * `store`: Data store that contains all the questions.
/// * `question`: Question to add to the data store.
async fn update_question(
  id: String,
  store: Store,
  question: Question,
) -> Result<impl warp::Reply, warp::Rejection> {
  match store.questions.write().await.get_mut(&QuestionId(id)) {
    Some(q) => {
      *q = question;
      Ok(warp::reply::with_status("Question updated", StatusCode::OK))
    },
    None => Err(warp::reject::custom(QError::QuestionNotFound)),
  }
} // end fn update_question()

/// Deletes an existing question with the given the ID and data store.
///
/// # Arguments
///
/// * `id`: ID (unique identifier) of the question to be deleted.
/// * `store`: Data store that contains all the questions.
async fn delete_question(
  id: String,
  store: Store,
) -> Result<impl warp::Reply, warp::Rejection> {
  match store.questions.write().await.remove(&QuestionId(id)) {
    Some(_) => {
      Ok(warp::reply::with_status(
        "Question deleted.",
        StatusCode::OK,
      ))
    },
    None => {
      Err(warp::reject::custom(QError::QuestionNotFound))
    }
  }
} // fn delete_question()

/// Represents the unique identifier (ID) of an answer.
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq, Hash)]
struct AnswerId(String);

/// Represents an answer to a given question.
#[derive(Debug, Clone, Serialize, Deserialize)]
struct Answer {
  /// Unique identifier (ID) of the answer.
  id: AnswerId,
  /// Text contents of the answer.
  content: String,
  /// Unique identifier (ID) of the question this answer belongs to.
  question_id: QuestionId,
} // end struct Answer

/// Adds a new answer with the given parameters to a data store.
///
/// # Arguments
///
/// * `store`: Data store for where answer will be saved.
/// * `params`: Set of parameters with data for adding a new answer.
async fn add_answer(
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

/// Represents the start and end index of a set of results.
#[derive(Debug)]
struct Pagination {
  /// Start index of a set of results.
  start: usize,
  /// End index of a set of results.
  end: usize,
} // end struct Pagination

/// Gets a pagination object from the given set of parameters.
///
/// Swaps the start and end indexes if the start index is greater than the end index.
///
/// # Arguments
///
/// * `params`: Parameters to limit the set of results to retrieve.
///
fn extract_pagination(params: HashMap<String, String>) -> Result<Pagination, QError> {
  if params.contains_key("start") && params.contains_key("end") {
    let start_index: usize = params
      .get("start")
      .unwrap()
      .parse::<usize>()
      .map_err(QError::ParseError)?;
    let end_index: usize = params
      .get("end")
      .unwrap()
      .parse::<usize>()
      .map_err(QError::ParseError)?;

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

  Err(QError::MissingParameters)
} // end fn extract_pagination()

/// Represents an error for processing query parameters.
#[derive(Debug)]
enum QError {
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
async fn return_error(rej: Rejection) -> Result<impl Reply, Rejection> {
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

/// Represents the data store for the application.
#[derive(Clone)]
struct Store {
  /// Collection of questions in the data store.
  questions: Arc<RwLock<HashMap<QuestionId, Question>>>,
  /// Collection of answers in the data store.
  answers: Arc<RwLock<HashMap<AnswerId, Answer>>>,
} // end struct Store

impl Store {
  /// Creates a new data store.
  fn new() -> Self {
    Self {
      questions: Arc::new(RwLock::new(Self::init())),
      answers: Arc::new(RwLock::new(HashMap::new())),
    }
  } // end fn new()

  /// Initializes the data store with available data.
  fn init() -> HashMap<QuestionId, Question> {
    let file: &str = include_str!("../questions.json");
    serde_json::from_str(file).expect("cannot read the questions.json file.")
  } // end fn init()
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
    .and(warp::query()) // adds a hash map of query parameters to the function specified in the last 'and_then()'
    .and(store_filter.clone()) // clone this filter
    .and_then(get_questions);

  let add_question = warp::post()
    .and(warp::path("questions"))
    .and(warp::path::end())
    .and(store_filter.clone())
    .and(warp::body::json())
    .and_then(add_question);

  let update_question = warp::put()
    .and(warp::path("questions"))
    .and(warp::path::param::<String>())
    .and(warp::path::end())
    .and(store_filter.clone())
    .and(warp::body::json()) // JSON Body with the question data.
    .and_then(update_question);

  let delete_question = warp::delete()
    .and(warp::path("questions"))
    .and(warp::path::param::<String>())
    .and(warp::path::end())
    .and(store_filter.clone())
    .and_then(delete_question);

  let add_answer = warp::post()
    .and(warp::path("answers"))
    .and(warp::path::end())
    .and(store_filter.clone())
    .and(warp::body::form())
    .and_then(add_answer);

  let routes = get_questions
    .or(add_question)
    .or(update_question)
    .or(delete_question)
    .or(add_answer)
    .with(cors)
    .recover(return_error);

  warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}
