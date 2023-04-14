use std::{error, fmt};

// TODO: this file is all boilerplate, would be interesting to look at a macro
// or derive-more (sp?) when time permits

// TODO: i also don't really know a good error strategy, kinda winging it.
// maybe this is too fine-grained?

#[derive(Debug, Clone)]
pub struct InvalidCacheDirError;

impl error::Error for InvalidCacheDirError {}

impl fmt::Display for InvalidCacheDirError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Invalid cache directory provided")
    }
}

#[derive(Debug, Clone)]
pub struct InvalidCacheKeyError;

impl error::Error for InvalidCacheKeyError {}

impl fmt::Display for InvalidCacheKeyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Invalid cache key provided")
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
pub struct InvalidProcessIDError;

impl error::Error for InvalidProcessIDError {}

impl fmt::Display for InvalidProcessIDError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Invalid process id provided")
    }
}

#[derive(Debug, Clone)]
pub struct InvalidElementIDError;

impl error::Error for InvalidElementIDError {}

impl fmt::Display for InvalidElementIDError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Invalid element id provided")
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

#[derive(Debug, Clone)]
pub enum GetElementUnitForProcessError {
    InvalidCacheDirError,
    InvalidCacheKeyError,
    InvalidProcessIDError,
}

impl error::Error for GetElementUnitForProcessError {}

impl fmt::Display for GetElementUnitForProcessError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // TODO: this isn't correct, want to call the error struct's fmt
        write!(f, "{:?}", self)
    }
}

#[derive(Debug, Clone)]
pub enum GetElementUnitForElementError {
    InvalidCacheDirError,
    InvalidCacheKeyError,
    InvalidProcessIDError,
    InvalidElementIDError,
}

impl error::Error for GetElementUnitForElementError {}

impl fmt::Display for GetElementUnitForElementError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // TODO: this isn't correct, want to call the error struct's fmt
        write!(f, "{:?}", self)
    }
}
