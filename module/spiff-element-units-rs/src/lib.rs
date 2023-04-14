pub mod error;
use error::CacheElementUnitsError;

// TODO: make these not String
pub fn cache_element_units(
    _cache_dir: String,
    _cache_key: String,
    _workflow_spec_json: String,
) -> Result<(), CacheElementUnitsError> {
    Ok(())
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
