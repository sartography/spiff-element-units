use pyo3::prelude::*;
use spiff_element_units_rs;

// TODO: String vs str when moving between Python

#[pyfunction]
fn cache_element_units(
    cache_dir: String,
    cache_key: String,
    workflow_spec_json: String,
) -> PyResult<()> {
    Ok(spiff_element_units_rs::cache_element_units(
        &cache_dir,
        &cache_key,
        &workflow_spec_json,
    )?)
}

#[pyfunction]
fn cached_element_unit_for_process(
    cache_dir: String,
    cache_key: String,
    process_id: String,
) -> PyResult<String> {
    Ok(spiff_element_units_rs::cached_element_unit_for_process(
        &cache_dir,
        &cache_key,
        &process_id,
    )?)
}

#[pyfunction]
fn cached_element_unit_for_element(
    cache_dir: String,
    cache_key: String,
    process_id: String,
    element_id: String,
) -> PyResult<String> {
    Ok(spiff_element_units_rs::cached_element_unit_for_element(
        &cache_dir,
        &cache_key,
        &process_id,
        &element_id,
    )?)
}

#[pymodule]
fn spiff_element_units(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(cache_element_units, m)?)?;
    m.add_function(wrap_pyfunction!(cached_element_unit_for_process, m)?)?;
    m.add_function(wrap_pyfunction!(cached_element_unit_for_element, m)?)?;
    Ok(())
}
