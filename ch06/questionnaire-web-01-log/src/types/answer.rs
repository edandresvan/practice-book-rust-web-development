use serde::{Deserialize, Serialize};

use crate::types::question::QuestionId;

/// Represents the unique identifier (ID) of an answer.
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq, Hash)]
pub struct AnswerId(pub String);

/// Represents an answer to a given question.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Answer {
  /// Unique identifier (ID) of the answer.
  pub id: AnswerId,
  /// Text contents of the answer.
  pub content: String,
  /// Unique identifier (ID) of the question this answer belongs to.
  pub question_id: QuestionId,
} // end struct Answer
