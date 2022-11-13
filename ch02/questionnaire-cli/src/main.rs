use std::{
  io::{Error, ErrorKind},
  str::FromStr,
  vec,
};

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
      true => Err(Error::new(ErrorKind::InvalidInput, "No id provided")),
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
      "{id}, title: {title}, content{content}, tags: {tags:?}",
      id = self.id,
      title = self.title,
      content = self.content,
      tags = self.tags
    )
  }
}

// impl std::fmt::Debug for Question {
//   fn fmt(
//     &self,
//     f: &mut std::fmt::Formatter<'_>,
//   ) -> std::fmt::Result {
//     write!(f, "{:?}", self.tags)
//   }
// }

fn main() {
  let question_1a: Question = Question::new(
    QuestionId("1".to_string()),
    "First Question".to_string(),
    "Content of question".to_string(),
    Some(vec!["fag".to_string()]),
  );
  println!("{}", question_1a);
  println!("......");
  println!("{:?}", question_1a);

  let question_1b: Question = Question::new(
    QuestionId::from_str("1").expect("No id provided"),
    "First Question".to_string(),
    "Content of question".to_string(),
    Some(vec!["faq".to_string()]),
  );

  println!("......");
  println!("{:?}", question_1b);
}
