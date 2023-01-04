use serde::Serialize;
use std::str::FromStr;
use warp::filters::cors::CorsForbidden;
use warp::http::Method;
use warp::http::StatusCode;
use warp::reject::Reject;
use warp::Filter;
use warp::Rejection;
use warp::Reply;

#[derive(Debug)]
struct InvalidId;
impl Reject for InvalidId {}

#[derive(Debug, Serialize)]
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

#[derive(Debug, Serialize)]
struct Question {
  id: QuestionId,
  title: String,
  content: String,
  tags: Option<Vec<String>>,
}

impl Question {
  fn new(
    id: QuestionId,
    title: String,
    content: String,
    tags: Option<Vec<String>>,
  ) -> Self {
    Self {
      id,
      title,
      content,
      tags,
    }
  }
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

async fn get_questions() -> Result<impl warp::Reply, warp::Rejection> {
  let question = Question::new(
    QuestionId::from_str("1").expect("No id was provided"),
    "First Question".to_string(),
    "Content of question".to_string(),
    Some(vec!["faq".to_string()]),
  );

  match question.id.0.parse::<i32>() {
    Ok(_) => Ok(warp::reply::json(&question)),
    Err(_) => Err(warp::reject::custom(InvalidId)),
  }
}

async fn return_error(rej: Rejection) -> Result<impl Reply, Rejection> {
  if let Some(error) = rej.find::<CorsForbidden>() {
    Ok(warp::reply::with_status(
      error.to_string(),
      StatusCode::FORBIDDEN,
    ))
  } else if let Some(InvalidId) = rej.find() {
    Ok(warp::reply::with_status(
      "No valid ID given".to_string(),
      StatusCode::UNPROCESSABLE_ENTITY,
    ))
  } else {
    // At this point, the possible rejection is that a path not found
    Ok(warp::reply::with_status(
      "Route not found".to_string(),
      StatusCode::NOT_FOUND,
    ))
  }
}

#[tokio::main]
async fn main() {
  let cors = warp::cors()
    .allow_any_origin()
    .allow_header("content-type")
    .allow_methods(&[Method::PUT, Method::DELETE, Method::GET, Method::POST]);

  let get_items = warp::get()
    .and(warp::path("questions"))
    .and(warp::path::end())
    .and_then(get_questions)
    .recover(return_error);

  let routes = get_items.with(cors);

  warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}
