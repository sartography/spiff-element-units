use pyo3::exceptions::PyValueError;
use pyo3::prelude::PyErr;
use spiff_element_units_rs::error::CacheElementUnitsError;

// TODO: https://pyo3.rs/v0.18.3/exception

pub struct PyCacheElementUnitsError(CacheElementUnitsError);

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
