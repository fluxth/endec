use encoding_rs::Encoding;
use pyo3::exceptions::PyLookupError;
use pyo3::prelude::*;

pub(crate) fn get_encoding_by_name(name: &str) -> PyResult<&'static Encoding> {
    Encoding::for_label(name.as_bytes()).ok_or(PyLookupError::new_err(format!(
        "unknown encoding: {}",
        name
    )))
}
