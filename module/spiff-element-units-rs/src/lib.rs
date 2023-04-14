pub mod error;
use error::CacheElementUnitsError;

pub fn cache_element_units(
    _cache_dir: &str,
    _cache_key: &str,
    _workflow_spec_json: &str,
) -> Result<(), CacheElementUnitsError> {
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn this_really_should_not_work() -> Result<(), CacheElementUnitsError> {
        let result = cache_element_units("", "", "")?;
        assert_eq!(result, ());
        Ok(())
    }
}
