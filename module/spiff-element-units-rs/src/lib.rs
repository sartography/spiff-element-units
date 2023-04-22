use std::error::Error;

mod cache;
mod domain;
mod element_units;
mod manifest;
mod reader;
mod writer;

use cache::entry::Type::{OriginalWorkflowSpecsJSON, OurWorkflowSpecsJSON};

//
// this is the public api. it is a thin waist on purpose to make other
// language bindings and caller interactions simple. we don't want to be
// chatty or pass complicated structures across boundaries.
//

//
// inside the lib, where possible, we return specific errors from functions.
// ok to return Box<dyn Error> if mulitple error types can originate from a
// internal function since we arn't explict at the api boundary anyway. this
// reduces a ton of boilerplate in the language binding(s) and is inline with
// the thin waist ideal.
//
type ApiResult<T> = Result<T, Box<dyn Error>>;

//
// construct and cache element units for a given workflow spec in json format
//
pub fn cache_element_units_for_workflow(
    cache_dir: &str,
    cache_key: &str,
    workflow_specs_json: &str,
) -> ApiResult<()> {
    // for now we are writing the original workflow specs json to the cache
    // even though we only return ours. this is to help keep an eye on things
    // and potientially help debug issues. it is not expected to be required forever.
    let entry_path =
        cache::created_path_for_entry(cache_dir, cache_key, OriginalWorkflowSpecsJSON)?;
    writer::write_string(&entry_path, workflow_specs_json)?;

    /*
    let our_workflow_spec = reader::read_string::<WorkflowSpec>(workflow_specs_json)?;
    let entry_path = cache::created_path_for_entry(cache_dir, cache_key, OurWorkflowSpecsJSON)?;
    writer::write(&entry_path, &our_workflow_spec)?;
    */

    let el_units = element_units::from_json_string(&workflow_specs_json)?;
    let manifest = manifest::from_element_units(&el_units);

    // TODO: cache the element unit that corresponds to each manifest entry
    // TODO: write the manifeset to cache

    /*
    let entry_path = cache::created_path_for_entry(cache_dir, cache_key, ElementUnitMap)?;
    writer::write(&entry_path, &element_unit_map)?;
    */

    Ok(())
}

//
// get a workflow which can run a previously cached element unit that contains
// the given element id.
//
pub fn workflow_from_cached_element_unit(
    cache_dir: &str,
    cache_key: &str,
    _element_id: &str,
) -> ApiResult<String> {
    // TODO: once the write part is finished above this needs to read the manifest and find
    // the entry that matches `element_id`. Then read from the path in the entry.
    let path = cache::path_for_entry(cache_dir, cache_key, OurWorkflowSpecsJSON);
    let contents = reader::read_to_string(&path)?;

    Ok(contents)
}
