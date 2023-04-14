use pyo3::prelude::*;
use pyo3::exceptions::PyValueError;
use spiff_element_units_rs::{
cache_element_units as lib_cache_element_units,
CacheElementUnitsError,
};

// TODO: move to errors.rs

struct PyCacheElementUnitsError(CacheElementUnitsError);

impl From<CacheElementUnitsError> for PyCacheElementUnitsError {
     fn from(e: CacheElementUnitsError) -> Self {
     	Self(e)
     }
}

impl From<PyCacheElementUnitsError> for PyErr {
     fn from(e: PyCacheElementUnitsError) -> Self {
     	PyValueError::new_err(e.0.to_string())
     }
}

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
