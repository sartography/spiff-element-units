use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use spiff_element_units_rs as lib;

// TODO: String vs str when moving between Python

// TODO: right now we just map the Box<dyn Error> to ValueError
// seems like other libs just map everything to RuntimeError
// - https://pyo3.rs/main/doc/pyo3/anyhow/index.html
//
// at somepoint if we want more granular exceptions will need more
// error boilerplate.

#[pyfunction]
fn cache_element_units_for_workflow(
    cache_dir: String,
    cache_key: String,
    workflow_specs_json: String,
) -> PyResult<()> {
    let result =
        lib::cache_element_units_for_workflow(&cache_dir, &cache_key, &workflow_specs_json)
            .map_err(|e| PyValueError::new_err(e.to_string()))?;

    Ok(result)
}

#[pyfunction]
fn workflow_from_cached_element_unit(
    cache_dir: String,
    cache_key: String,
    process_id: String,
    element_id: String,
    capabilities: u64,
) -> PyResult<String> {
    let result = lib::workflow_from_cached_element_unit(
        &cache_dir,
        &cache_key,
        &process_id,
        &element_id,
        capabilities,
    )
    .map_err(|e| PyValueError::new_err(e.to_string()))?;

    Ok(result)
}

#[pymodule]
fn spiff_element_units(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(cache_element_units_for_workflow, m)?)?;
    m.add_function(wrap_pyfunction!(workflow_from_cached_element_unit, m)?)?;
    Ok(())
}
