pub mod error;
use error::{CacheElementUnitsError, GetElementUnitForElementError, GetElementUnitForProcessError};

// this is the public api. it is a thin waist on purpose to make other
// language bindings simple. we don't want to be chatty or pass complicated
// structures across boundaries.

//
// construct and cache element units for a given workflow spec in json format
//
pub fn cache_element_units(
    _cache_dir: &str,
    _cache_key: &str,
    _workflow_spec_json: &str,
) -> Result<(), CacheElementUnitsError> {
    Ok(())
}

//
// get the element unit required to start the process
//
pub fn get_element_unit_for_process(
    _cache_dir: &str,
    _cache_key: &str,
    _process_id: &str,
) -> Result<String, GetElementUnitForProcessError> {
    Ok("".to_string())
}

//
// get the element unit required to resume a process at a given element
//
pub fn get_element_unit_for_element(
    _cache_dir: &str,
    _cache_key: &str,
    _process_id: &str,
    _element_id: &str,
) -> Result<String, GetElementUnitForElementError> {
    Ok("".to_string())
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
