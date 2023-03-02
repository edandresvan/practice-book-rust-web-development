use serde::{Deserialize, Serialize};

/// Represents a valid identifier (ID) for a question.
#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq, Hash)]
pub struct QuestionId(pub String);

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
pub struct Question {
  /// Identifier of the question.
  pub id: QuestionId,
  /// Title of the question.
  pub title: String,
  /// Text contents of the question.
  pub content: String,
  /// List of tags for the question.
  pub tags: Option<Vec<String>>,
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
