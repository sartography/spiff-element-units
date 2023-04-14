use std::{error, fmt};

// TODO: move to errors.rs and maybe re-export from here

#[derive(Debug, Clone)]
pub struct InvalidCacheDirError;

impl error::Error for InvalidCacheDirError {}

impl fmt::Display for InvalidCacheDirError {
     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
     	write!(f, "Invalid cache directory provided")
     }
}

#[derive(Debug, Clone)]
pub struct InvalidWorkflowSpecJSONError;

impl error::Error for InvalidWorkflowSpecJSONError {}

impl fmt::Display for InvalidWorkflowSpecJSONError {
     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
     	write!(f, "Invalid workflow spec json provided")
     }
}

#[derive(Debug, Clone)]
pub enum CacheElementUnitsError {
    InvalidCacheDirError,
    InvalidWorkflowSpecJSONError,
}

impl error::Error for CacheElementUnitsError {}

impl fmt::Display for CacheElementUnitsError {
     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
     	// TODO: this isn't correct, want to call the error struct's fmt
     	write!(f, "{:?}", self)
     }
}

pub fn cache_element_units(
    _cache_dir: String,
    _cache_key: String,
    _workflow_spec_json: String,
) -> Result<(), CacheElementUnitsError> {
    Ok(())
}

// TODO: remove
pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn this_really_should_not_work() -> Result<(), CacheElementUnitsError> {
        let result = cache_element_units("".to_string(), "".to_string(), "".to_string())?;
        assert_eq!(result, ());
	Ok(())
    }
}
