mod routes;
mod store;
mod types;
use handle_errors::errors::return_error;
use warp::http::Method;
use warp::Filter;

use crate::routes::answer::add_answer;
use crate::routes::question::{
  add_question, delete_question, get_questions, update_question,
};
use crate::store::Store;

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
