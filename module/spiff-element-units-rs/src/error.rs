use std::{error, fmt};

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
