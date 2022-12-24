use std::str::FromStr;
use warp::Filter;

#[derive(Debug)]
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

#[derive(Debug)]
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

#[tokio::main]
async fn main() {
  /* let question = Question::new(
    QuestionId::from_str("1").expect("No ID provided."),
    "First Question".to_string(),
    "Content of question".to_string(),
    Some(vec!["faq".to_string()]),
  );
  println!("{:?}", &question); */

  let hello = warp::get().map(|| format!("Hello, Rust World!"));

  warp::serve(hello).run(([127, 0, 0, 1], 3030)).await;
}
