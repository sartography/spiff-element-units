use pyo3::prelude::*;
use spiff_element_units_rs as lib;

// TODO: String vs str when moving between Python

#[pyfunction]
fn cache_element_units_for_workflow(
    cache_dir: String,
    cache_key: String,
    workflow_specs_json: String,
) -> PyResult<()> {
    Ok(lib::cache_element_units_for_workflow(
        &cache_dir,
        &cache_key,
        &workflow_specs_json,
    )?)
}

#[pyfunction]
fn workflow_from_cached_element_unit(
    cache_dir: String,
    cache_key: String,
    element_id: String,
) -> PyResult<String> {
    Ok(lib::workflow_from_cached_element_unit(
        &cache_dir,
        &cache_key,
        &element_id,
    )?)
}

#[pymodule]
fn spiff_element_units(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(cache_element_units_for_workflow, m)?)?;
    m.add_function(wrap_pyfunction!(workflow_from_cached_element_unit, m)?)?;
    Ok(())
}
