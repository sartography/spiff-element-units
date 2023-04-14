use pyo3::prelude::*;
use spiff_element_units_rs::cache_element_units as lib_cache_element_units;

#[pyfunction]
fn cache_element_units(
    cache_dir: String,
    cache_key: String,
    workflow_spec_json: String,
) -> PyResult<()> {
    lib_cache_element_units(cache_dir, cache_key, workflow_spec_json)
}

#[pymodule]
fn spiff_element_units(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(cache_element_units, m)?)?;
    Ok(())
}
