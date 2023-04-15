
mod cache;

// this is the public api. it is a thin waist on purpose to make other
// language bindings simple. we don't want to be chatty or pass complicated
// structures across boundaries.

//
// construct and cache element units for a given workflow spec in json format
//
pub fn create_element_units(
    cache_dir: &str,
    cache_key: &str,
    workflow_spec_json: &str,
) -> std::io::Result<()> {
  // TODO: eventually we will want to validate the workflow_spec_json
  // before caching.
  // TODO: right now we are not decomposing at all to get the integration
  // started with the backend
    cache::write_workflow_spec(cache_dir, cache_key, workflow_spec_json)
}

//
// get the element unit required to start the process
//
pub fn element_unit_for_process(
    cache_dir: &str,
    cache_key: &str,
    _process_id: &str,
) -> std::io::Result<String> {
  // TODO: right now we are just returning back the whole workflow spec json to
  // get the itegration ball rolling
  // TODO: eventually we will want to validate the workflow_spec_json
  // before returning.
    cache::read_workflow_spec(cache_dir, cache_key)
}

//
// get the element unit required to resume a process at a given element
//
pub fn element_unit_for_element(
    cache_dir: &str,
    cache_key: &str,
    _process_id: &str,
    _element_id: &str,
) -> std::io::Result<String> {
  // TODO: right now we are just returning back the whole workflow spec json to
  // get the itegration ball rolling
  // TODO: eventually we will want to validate the workflow_spec_json
  // before returning.
    cache::read_workflow_spec(cache_dir, cache_key)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn this_really_should_not_work() -> std::io::Result<()> {
        let result = create_element_units("", "", "")?;
        assert_eq!(result, ());
        Ok(())
    }
}
