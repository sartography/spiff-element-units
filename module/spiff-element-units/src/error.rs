use pyo3::exceptions::PyValueError;
use pyo3::prelude::PyErr;
use spiff_element_units_rs::error::{
    CacheElementUnitsError, GetElementUnitForElementError, GetElementUnitForProcessError,
};

// TODO: https://pyo3.rs/v0.18.3/exception

// TODO: same comment about boilerplate here as rs-lib errors
// TODO: for the Python callers would be good to have a base exception

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

pub struct PyGetElementUnitForProcessError(GetElementUnitForProcessError);

impl From<GetElementUnitForProcessError> for PyGetElementUnitForProcessError {
    fn from(e: GetElementUnitForProcessError) -> Self {
        Self(e)
    }
}

impl From<PyGetElementUnitForProcessError> for PyErr {
    fn from(e: PyGetElementUnitForProcessError) -> Self {
        PyValueError::new_err(e.0.to_string())
    }
}

pub struct PyGetElementUnitForElementError(GetElementUnitForElementError);

impl From<GetElementUnitForElementError> for PyGetElementUnitForElementError {
    fn from(e: GetElementUnitForElementError) -> Self {
        Self(e)
    }
}

impl From<PyGetElementUnitForElementError> for PyErr {
    fn from(e: PyGetElementUnitForElementError) -> Self {
        PyValueError::new_err(e.0.to_string())
    }
}
