use serde::{Deserialize, Serialize};

/// Represents a valid identifier (ID) for a question.
#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq, Hash)]
pub struct QuestionId(pub i32);

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
      false => match id.parse::<i32>() {
        Ok(value) => Ok(QuestionId(value)),
        Err(err) => Err(std::io::Error::new(
          std::io::ErrorKind::InvalidInput,
          format!("ID is not an integer i32. {}", err),
        )),
      },
      true => Err(std::io::Error::new(
        std::io::ErrorKind::InvalidInput,
        "No ID provided",
      )),
    }
  }
}

/// Represents a question posted in the system.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Question {
  /// Identifier of the question.
  pub id: QuestionId,
  /// Title of the question.
  pub title: String,
  /// Text contents of the question.
  pub content: String,
  /// List of tags for the question.
  pub tags: Option<Vec<String>>,
} // end Question struct

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


/// Represents a new question that will be posted in the system.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewQuestion {
  /// Title of the question.
  pub title: String,
  /// Text contents of the question.
  pub content: String,
  /// List of tags for the question.
  pub tags: Option<Vec<String>>,
} // end NewQuestion struct

