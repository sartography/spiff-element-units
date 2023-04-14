use pyo3::prelude::*;
use spiff_element_units_rs::{
    cache_element_units as lib_cache_element_units,
    get_element_unit_for_element as lib_get_element_unit_for_element,
    get_element_unit_for_process as lib_get_element_unit_for_process,
};

mod error;
use error::{
    PyCacheElementUnitsError, PyGetElementUnitForElementError, PyGetElementUnitForProcessError,
};

// TODO: String vs str in the interface

#[pyfunction]
fn cache_element_units(
    cache_dir: String,
    cache_key: String,
    workflow_spec_json: String,
) -> Result<(), PyCacheElementUnitsError> {
    Ok(lib_cache_element_units(
        &cache_dir,
        &cache_key,
        &workflow_spec_json,
    )?)
}

#[pyfunction]
fn get_element_unit_for_process(
    cache_dir: String,
    cache_key: String,
    process_id: String,
) -> Result<String, PyGetElementUnitForProcessError> {
    Ok(lib_get_element_unit_for_process(
        &cache_dir,
        &cache_key,
        &process_id,
    )?)
}

#[pyfunction]
fn get_element_unit_for_element(
    cache_dir: String,
    cache_key: String,
    process_id: String,
    element_id: String,
) -> Result<String, PyGetElementUnitForElementError> {
    Ok(lib_get_element_unit_for_element(
        &cache_dir,
        &cache_key,
        &process_id,
        &element_id,
    )?)
}

#[pymodule]
fn spiff_element_units(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(cache_element_units, m)?)?;
    m.add_function(wrap_pyfunction!(get_element_unit_for_process, m)?)?;
    m.add_function(wrap_pyfunction!(get_element_unit_for_element, m)?)?;
    Ok(())
}
