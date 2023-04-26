use std::error::Error;
use std::iter::zip;

mod cache;
mod config;
mod domain;
mod element_units;
mod manifest;
mod reader;
mod writer;

use cache::entry::Type as CacheEntryType;
use cache::manifest::Manifest;
use element_units::ElementUnit;

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
    let entry_path = cache::created_path_for_entry(
        cache_dir,
        cache_key,
        CacheEntryType::OriginalWorkflowSpecsJSON,
    )?;
    writer::write_string(&entry_path, workflow_specs_json)?;

    let el_units_by_process_id = element_units::from_json_string(&workflow_specs_json)?;

    for (process_id, el_units) in el_units_by_process_id.iter() {
        let manifest = manifest::from_element_units(&el_units);
        let el_units_and_manifest_entries = zip(&el_units.items, &manifest.items);

        for (el_unit, manifest_entry) in el_units_and_manifest_entries {
            let entry_path = cache::created_path_for_entry(
                cache_dir,
                cache_key,
                CacheEntryType::ManifestEntry(&manifest_entry.sha2),
            )?;
            writer::write(&entry_path, el_unit)?;
        }

        let entry_path = cache::created_path_for_entry(
            cache_dir,
            cache_key,
            CacheEntryType::Manifest(process_id),
        )?;
        writer::write(&entry_path, &manifest)?;
    }

    Ok(())
}

//
// get a workflow which can run a previously cached element unit that contains
// the given element id.
//
pub fn workflow_from_cached_element_unit(
    cache_dir: &str,
    cache_key: &str,
    process_id: &str,
    element_id: &str,
) -> ApiResult<String> {
    let entry_path =
        cache::path_for_entry(cache_dir, cache_key, CacheEntryType::Manifest(process_id));
    let manifest = reader::read::<Manifest>(&entry_path)?;
    let manifest_entry = manifest
        .last_item_for_key(element_id.to_string())
        .ok_or("Element unit not found.")?;

    let entry_path = cache::path_for_entry(
        cache_dir,
        cache_key,
        CacheEntryType::ManifestEntry(&manifest_entry.sha2),
    );
    let element_unit = reader::read::<ElementUnit>(&entry_path)?;
    let workflow_spec = element_unit.to_workflow_spec();

    let contents = writer::write_to_string(workflow_spec)?;

    Ok(contents)
}
