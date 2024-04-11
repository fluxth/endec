use pyo3::exceptions::PyLookupError;
use pyo3::prelude::*;

pyo3::import_exception!(endec.exceptions, EncodeError);
pyo3::import_exception!(endec.exceptions, DecodeError);

pub(crate) fn encoding_lookup_failed(encoding_name: &str) -> PyErr {
    PyLookupError::new_err(format!("unknown encoding: {}", encoding_name.trim()))
}

pub(crate) fn error_handler_lookup_failed(error_handler_name: &str) -> PyErr {
    PyLookupError::new_err(format!(
        "unknown error handler name '{}'",
        error_handler_name.trim()
    ))
}

pub(crate) fn bom_handler_lookup_failed(bom_handler_name: &str) -> PyErr {
    PyLookupError::new_err(format!(
        "unknown byte-order mark handler name '{}'",
        bom_handler_name.trim()
    ))
}

pub(crate) fn encode_failed(used_encoding_name: &str, input_str: &str) -> PyErr {
    EncodeError::new_err((used_encoding_name.trim().to_owned(), input_str.to_owned()))
}

pub(crate) fn decode_failed(used_encoding_name: &str, input_bytes: &[u8]) -> PyErr {
    DecodeError::new_err((used_encoding_name.trim().to_owned(), input_bytes.to_owned()))
}
