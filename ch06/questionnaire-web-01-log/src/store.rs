use std::{collections::HashMap, sync::Arc};
use tokio::sync::RwLock;

use crate::types::answer::{Answer, AnswerId};
use crate::types::question::{Question, QuestionId};

/// Represents the data store for the application.
#[derive(Clone)]
pub struct Store {
  /// Collection of questions in the data store.
  pub questions: Arc<RwLock<HashMap<QuestionId, Question>>>,
  /// Collection of answers in the data store.
  pub answers: Arc<RwLock<HashMap<AnswerId, Answer>>>,
} // end struct Store

impl Store {
  /// Creates a new data store.
  pub fn new() -> Self {
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
