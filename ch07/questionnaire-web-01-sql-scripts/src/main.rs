use handle_errors::errors::return_error;
use warp::http::Method;
use warp::Filter;

use tracing_subscriber::fmt::format::FmtSpan;

mod routes;
mod store;
mod types;

use crate::routes::answer::add_answer;
use crate::routes::question::{
  add_question, delete_question, get_questions, update_question,
};
use crate::store::Store;

#[tokio::main]
async fn main() {
  /*  Log Fachades */

  // Filter configured with a log level in this case 'error'.
  // Note first the crate name
  let log_filter = std::env::var("RUST_LOG")
    .unwrap_or_else(|_| "questionnaire_web=info,warp=error".to_owned());

  // Start the tracing subscriber
  tracing_subscriber::fmt()
    // Use the filter to record traces
    .with_env_filter(log_filter)
    // Record events when each span closes
    .with_span_events(FmtSpan::CLOSE)
    .init();

  // Create the data store
  let url: &str = "postgres://firstdev:mypassword@localhost:5432/rustwebdev";
  let store = Store::new(url).await;
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
    .and_then(get_questions)
    .with(warp::trace(|info| {
      tracing::info_span!("get_questions request", 
      method= %info.method(), path = %info.path(), 
      id = %uuid::Uuid::new_v4(),)
    }));

  let add_question = warp::post()
    .and(warp::path("questions"))
    .and(warp::path::end())
    .and(store_filter.clone())
    .and(warp::body::json())
    .and_then(add_question);

  let update_question = warp::put()
    .and(warp::path("questions"))
    .and(warp::path::param::<i32>())
    .and(warp::path::end())
    .and(store_filter.clone())
    .and(warp::body::json()) // JSON Body with the question data.
    .and_then(update_question);

  let delete_question = warp::delete()
    .and(warp::path("questions"))
    .and(warp::path::param::<i32>())
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
    .with(warp::trace::request())
    .recover(return_error);

  warp::serve(routes).run(([127, 0, 0, 1], 3030)).await
}
