use sqlx::postgres::{PgPoolOptions, PgRow};
use sqlx::PgPool;
// This trait allows working with row results
use sqlx::Row;

use crate::types::answer::{Answer, AnswerId, NewAnswer};
use crate::types::question::{NewQuestion, Question, QuestionId};

use handle_errors::errors::QError;

/// Represents the data store for the application.
#[derive(Debug, Clone)]
pub struct Store {
  /// Pool for database connections.
  pub connection: PgPool,
} // end struct Store

impl Store {
  /// Creates a new data store.
  ///
  /// # Arguments
  ///
  /// * `db_url`: URL of the database server.
  pub async fn new(db_url: &str) -> Self {
    let db_pool = match PgPoolOptions::new()
      .max_connections(5)
      .connect(db_url)
      .await
    {
      Ok(pool) => pool,
      Err(err) => panic!("Database connection failed. {}", err),
    };

    Self {
      connection: db_pool,
    }
  } // end fn new()

  /// Gets the collection of questions.
  ///
  /// # Arguments
  ///
  /// * `offset`: Start index of a set of results, i.e. offset.
  /// * `limit`: Amount of elements of the set of results. i.e. limit. End index of a set of results.
  pub async fn get_questions(
    &self,
    offset: i32,
    limit: Option<i32>,
  ) -> Result<Vec<Question>, QError> {
    let db_query_set = sqlx::query("SELECT * from questions LIMIT $1 OFFSET $2")
      .bind(limit)
      .bind(offset)
      .map(|row: PgRow| Question {
        id: QuestionId(row.get("id")),
        title: row.get("title"),
        content: row.get("content"),
        tags: row.get("tags"),
      })
      .fetch_all(&self.connection)
      .await;

    match db_query_set {
      Ok(questions) => Ok(questions),
      Err(err) => {
        tracing::event!(tracing::Level::ERROR, "{:?}", err);
        Err(QError::DatabaseQueryError(err))
      }
    }
  } // end fn get_questions()

  /// Adds a new question to the system.
  ///
  /// # Arguments
  ///
  /// * `question`: Question to be added.
  pub async fn add_question(
    &self,
    question: NewQuestion,
  ) -> Result<Vec<Question>, QError> {
    match sqlx::query(
      r#"INSERT INTO questions (title, content, tags) 
      VALUES ($1, $2, $3) 
      RETURNING id, title, content, tags"#,
    )
    .bind(question.title)
    .bind(question.content)
    .bind(question.tags)
    .map(|row: PgRow| Question {
      id: QuestionId(row.get("id")),
      title: row.get("title"),
      content: row.get("content"),
      tags: row.get("tags"),
    })
    .fetch_all(&self.connection)
    .await
    {
      Ok(questions) => Ok(questions),
      Err(err) => {
        tracing::event!(tracing::Level::ERROR, "{:?}", err);
        Err(QError::DatabaseQueryError(err))
      }
    }
  } // end fn add_question()

  /// Updates an existing question in the datastore.
  ///
  /// # Arguments
  ///
  /// * `question`: Question data.
  /// * `id`: Unique identifier (ID) of the question.
  pub async fn update_question(
    &self,
    question: Question,
    id: i32,
  ) -> Result<Vec<Question>, QError> {
    match sqlx::query(
      r#"UPDATE questions 
      SET title = $1, content = $2, tags = $3 
      WHERE id = $4 
      RETURNING id, title, content, tags"#,
    )
    .bind(question.title)
    .bind(question.content)
    .bind(question.tags)
    .bind(id)
    .map(|row: PgRow| Question {
      id: QuestionId(row.get("id")),
      title: row.get("content"),
      content: row.get("content"),
      tags: row.get("tags"),
    })
    .fetch_all(&self.connection)
    .await
    {
      Ok(questions) => Ok(questions),
      Err(err) => {
        tracing::event!(tracing::Level::ERROR, "{:?}", err);
        Err(QError::DatabaseQueryError(err))
      }
    }
  } // end fn update_question()

  /// Deletes the questions specified by the given id from the datastore.
  ///
  /// # Arguments
  ///
  /// * `id`: Unique identifier (ID) of the question to be deleted.
  pub async fn delete_question(
    &self,
    id: i32,
  ) -> Result<u64, QError> {
    match sqlx::query(
      r#"DELETE FROM questions 
      WHERE id = $1"#,
    )
    .bind(id)
    .execute(&self.connection)
    .await
    {
      Ok(result) => Ok(result.rows_affected()),
      Err(err) => {
        tracing::event!(tracing::Level::ERROR, "{:?}", err);
        Err(QError::DatabaseQueryError(err))
      }
    }
  } // end fn delete_question()

  /// Adds a new answer to the datastore.
  ///
  /// # Arguments
  ///
  /// * `answer`: Answer to be added.
  pub async fn add_answer(
    &self,
    answer: NewAnswer,
  ) -> Result<Vec<Answer>, QError> {
    match sqlx::query(
      r#"INSERT INTO answers (content, question_id) 
      VALUES ($1, $2)"#,
    )
    .bind(answer.content)
    .bind(answer.question_id.0)
    .map(|row: PgRow| Answer {
      id: AnswerId(row.get("id")),
      content: row.get("content"),
      question_id: QuestionId(row.get("question_id")),
    })
    .fetch_all(&self.connection)
    .await
    {
      Ok(answers) => Ok(answers),
      Err(err) => {
        tracing::event!(tracing::Level::ERROR, "{:?}", err);
        Err(QError::DatabaseQueryError(err))
      }
    }
  } // fn add_answer()
}
