use std::collections::HashMap;

use crate::errors::Error;

/// Pagination struct which is getting extract
/// from query params
#[derive(Default, Debug, PartialEq)]
pub struct Pagination {
    /// The index of the last item which has to be returned
    pub limit: Option<i32>,
    /// The index of the first item which has to be returned
    pub offset: i32,
}

#[derive(Default, Debug, PartialEq)]
pub struct PaginationForJob {
    /// The index of the last item which has to be returned
    pub limit: Option<i32>,
    /// The index of the first item which has to be returned
    pub offset: i32,
    pub job_id: i32,
}

/// Extract query parameters from the `/questions` route
/// # Example query
/// GET requests to this route can have a pagination attached so we just
/// return the questions we need
/// `/questions?start=1&end=10`
/// # Example usage
/// ```rust
/// use std::collections::HashMap;
/// use rust_web_dev::types::pagination::extract_pagination;
///
/// let mut query = HashMap::new();
/// query.insert("limit".to_string(), "1".to_string());
/// query.insert("offset".to_string(), "10".to_string());
/// let p = extract_pagination(query).unwrap();
/// assert_eq!(p.limit, Some(1));
/// assert_eq!(p.offset, 10);
/// ```

impl Pagination {
    pub fn extract_pagination(params: HashMap<String, String>) -> Result<Pagination, Error> {
        // Could be improved in the future
        if params.contains_key("limit") && params.contains_key("offset") {
            return Ok(Pagination {
                // Takes the "limit" parameter in the query and tries to convert it to a number
                limit: Some(params.get("limit").unwrap().parse().map_err(Error::Parse)?),
                // Takes the "offset" parameter in the query and tries to convert it to a number
                offset: params
                    .get("offset")
                    .unwrap()
                    .parse()
                    .map_err(Error::Parse)?,
            });
        }
        Err(Error::MissingParameters)
    }
}

impl PaginationForJob {
    pub fn extract_pagination_job(
        params: HashMap<String, String>,
    ) -> Result<PaginationForJob, Error> {
        if params.contains_key("limit")
            && params.contains_key("offset")
            && params.contains_key("jobId")
        {
            return Ok(PaginationForJob {
                // Takes the "limit" parameter in the query and tries to convert it to a number
                limit: Some(params.get("limit").unwrap().parse().map_err(Error::Parse)?),
                // Takes the "offset" parameter in the query and tries to convert it to a number
                offset: params
                    .get("offset")
                    .unwrap()
                    .parse()
                    .map_err(Error::Parse)?,
                // Takes the "jobId" parameter in the query and tries to convert it to a number
                job_id: params.get("jobId").unwrap().parse().map_err(Error::Parse)?,
            });
        }
        Err(Error::MissingParameters)
    }
}

#[cfg(test)]
mod pagination_tests {
    use super::{Error, HashMap, Pagination};

    #[test]
    fn valid_pagination() {
        let mut params = HashMap::new();
        params.insert(String::from("limit"), String::from("1"));
        params.insert(String::from("offset"), String::from("1"));
        // let pagination_result = <Pagination as PaginationMethods>::extract_pagination(params);
        let pagination_result = Pagination::extract_pagination(params);
        let expected = Pagination {
            limit: Some(1),
            offset: 1,
        };
        assert_eq!(pagination_result.unwrap(), expected);
    }

    #[test]
    fn missing_offset_parameter() {
        let mut params = HashMap::new();
        params.insert(String::from("limit"), String::from("1"));

        let pagination_result = format!("{}", Pagination::extract_pagination(params).unwrap_err());
        let expected = format!("{}", Error::MissingParameters);

        assert_eq!(pagination_result, expected);
    }

    #[test]
    fn missing_limit_paramater() {
        let mut params = HashMap::new();
        params.insert(String::from("offset"), String::from("1"));

        let pagination_result = format!("{}", Pagination::extract_pagination(params).unwrap_err());
        let expected = format!("{}", Error::MissingParameters);

        assert_eq!(pagination_result, expected);
    }

    #[test]
    fn wrong_offset_type() {
        let mut params = HashMap::new();
        params.insert(String::from("limit"), String::from("1"));
        params.insert(String::from("offset"), String::from("C"));
        let pagination_result = format!("{}", Pagination::extract_pagination(params).unwrap_err());

        let expected = String::from("Can't parse parameter: invalid digit found in string");

        assert_eq!(pagination_result, expected);
    }

    #[test]
    fn wrong_limit_type() {
        let mut params = HashMap::new();
        params.insert(String::from("limit"), String::from("C"));
        params.insert(String::from("offset"), String::from("1"));
        let pagination_result = format!("{}", Pagination::extract_pagination(params).unwrap_err());

        let expected = String::from("Can't parse parameter: invalid digit found in string");

        assert_eq!(pagination_result, expected);
    }
}
