mod cache;
mod domain;
mod parser;

// this is the public api. it is a thin waist on purpose to make other
// language bindings and caller interactions simple. we don't want to be
// chatty or pass complicated structures across boundaries.

//
// construct and cache element units for a given workflow spec in json format
//
pub fn cache_element_units_for_workflow(
    cache_dir: &str,
    cache_key: &str,
    workflow_specs_json: &str,
) -> std::io::Result<()> {
    // TODO: eventually we will want to validate the workflow_specs_json
    // before caching.
    let _workflow_spec = parser::parse_str(workflow_specs_json);

    // TODO: right now we are not decomposing at all to get the integration
    // started with the backend
    cache::write_workflow_specs(cache_dir, cache_key, workflow_specs_json)
}

//
// get a workflow which can run a previously cached element unit that contains
// the given element id.
//
pub fn workflow_from_cached_element_unit(
    cache_dir: &str,
    cache_key: &str,
    _element_id: &str,
) -> std::io::Result<String> {
    // TODO: right now we are just returning back the whole workflow specs json to
    // get the itegration ball rolling
    cache::read_workflow_specs(cache_dir, cache_key)
}
