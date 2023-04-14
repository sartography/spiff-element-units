use pyo3::prelude::PyErr;
use pyo3::exceptions::PyValueError;
use spiff_element_units_rs::error::CacheElementUnitsError;

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
