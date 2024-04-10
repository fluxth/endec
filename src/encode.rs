use pyo3::exceptions::PyLookupError;
use pyo3::prelude::*;
use pyo3::types::PyBytes;

use crate::exceptions::EncodeError;
use crate::utils::get_encoding_by_name;

#[pyfunction]
#[pyo3(signature = (input_str, /, encoding = "utf-8", errors = "strict"))]
pub(crate) fn encode(
    py: Python,
    input_str: &str,
    encoding: &str,
    errors: &str,
) -> PyResult<PyObject> {
    let (out, used_encoding, has_unmappable) = get_encoding_by_name(encoding)?.encode(input_str);
    let convert_to_pybytes = || PyBytes::new_bound(py, &out).into();

    if !has_unmappable {
        return Ok(convert_to_pybytes());
    }

    match errors {
        "strict" => Err(EncodeError::new_err((
            used_encoding.name().to_owned(),
            input_str.to_owned(),
        ))),
        "xmlcharrefreplace" => Ok(convert_to_pybytes()),
        _ => Err(PyLookupError::new_err(format!(
            "unknown error handler name '{}'",
            errors
        ))),
    }
}
