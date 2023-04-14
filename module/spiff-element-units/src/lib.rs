use pyo3::prelude::*;
use spiff_element_units_rs::cache_element_units as lib_cache_element_units;

mod error;
use error::PyCacheElementUnitsError;

#[pyfunction]
fn cache_element_units(
    cache_dir: String,
    cache_key: String,
    workflow_spec_json: String,
) -> Result<(), PyCacheElementUnitsError> {
    Ok(lib_cache_element_units(cache_dir, cache_key, workflow_spec_json)?)
}

#[pymodule]
fn spiff_element_units(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(cache_element_units, m)?)?;
    Ok(())
}
