use std::collections::HashMap;

use handle_errors::errors::QError;

/// Represents the start and end index of a set of results.
#[derive(Default, Debug)]
pub struct Pagination {
  /// Start index of a set of results, i.e. offset.
  pub offset: i32,
  /// Amount of elements of the set of results. i.e. limit. End index of a set of results.
  pub limit: Option<i32>,
} // end struct Pagination

/// Gets a pagination object from the given set of parameters.
///
/// # Arguments
///
/// * `params`: Parameters to limit the set of results to retrieve.
///
/// # Example Usage
///
/// ```rust  
/// let mut query = HashMap::new();
/// query.insert("offset".to_string(), "1".to_string());
/// query.insert("limit").to_string(), "20".to_string());
///
/// let pagination = types::pagination::extract_pagination(query).unwrap;
/// assert_eq!(pagination.offset, 1);
/// assert_eq!(pagination.limit, 20);
/// ```
pub fn extract_pagination(params: HashMap<String, String>) -> Result<Pagination, QError> {
  if params.contains_key("offset") && params.contains_key("limit") {
    let offset_value: i32 = params
      .get("offset")
      .unwrap()
      .parse::<i32>()
      .map_err(QError::ParseError)?;
    let limit_value: i32 = params
      .get("limit")
      .unwrap()
      .parse::<i32>()
      .map_err(QError::ParseError)?;

    let pagination = Pagination {
      offset: offset_value,
      limit: Some(limit_value),
    };

    return Ok(pagination);
  }

  Err(QError::MissingParameters)
} // end fn extract_pagination()
