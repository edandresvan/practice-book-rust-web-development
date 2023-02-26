use std::collections::HashMap;

use handle_errors::errors::QError;

/// Represents the start and end index of a set of results.
#[derive(Debug)]
pub struct Pagination {
  /// Start index of a set of results.
  pub start: usize,
  /// End index of a set of results.
  pub end: usize,
} // end struct Pagination

/// Gets a pagination object from the given set of parameters.
///
/// Swaps the start and end indexes if the start index is greater than the end index.
///
/// # Arguments
///
/// * `params`: Parameters to limit the set of results to retrieve.
///
pub fn extract_pagination(params: HashMap<String, String>) -> Result<Pagination, QError> {
  if params.contains_key("start") && params.contains_key("end") {
    let start_index: usize = params
      .get("start")
      .unwrap()
      .parse::<usize>()
      .map_err(QError::ParseError)?;
    let end_index: usize = params
      .get("end")
      .unwrap()
      .parse::<usize>()
      .map_err(QError::ParseError)?;

    // Swap start and end indexes if the start index is greater than the end index
    let (start_index, end_index) = if start_index > end_index {
      (end_index, start_index)
    } else {
      (start_index, end_index)
    };

    let pagination = Pagination {
      start: start_index,
      end: end_index,
    };

    return Ok(pagination);
  }

  Err(QError::MissingParameters)
} // end fn extract_pagination()
