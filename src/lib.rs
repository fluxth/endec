use pyo3::{
    exceptions::{PyLookupError, PyValueError},
    prelude::*,
    types::{PyBytes, PyString},
};

fn get_codec(name: &str) -> PyResult<&'static encoding_rs::Encoding> {
    encoding_rs::Encoding::for_label(name.as_bytes()).ok_or(PyLookupError::new_err(format!(
        "unknown encoding: {}",
        name
    )))
}

#[pyfunction]
#[pyo3(signature = (input_str, /, encoding = "utf-8", errors = "strict"))]
fn encode(py: Python, input_str: &str, encoding: &str, errors: &str) -> PyResult<PyObject> {
    let (out, _, has_unmappable) = get_codec(encoding)?.encode(input_str);
    let convert_to_pybytes = || PyBytes::new(py, &out).into();

    if !has_unmappable {
        return Ok(convert_to_pybytes());
    }

    match errors {
        "strict" => Err(PyValueError::new_err("Cannot encode")),
        "xmlcharrefreplace" => Ok(convert_to_pybytes()),
        _ => Err(PyLookupError::new_err(format!(
            "unknown error handler name '{}'",
            errors
        ))),
    }
}

#[pyfunction]
#[pyo3(signature = (input_bytes, /, encoding = "utf-8", errors = "strict"))]
fn decode<'py>(
    py: Python<'py>,
    input_bytes: &'py [u8],
    encoding: &'py str,
    errors: &'py str,
) -> PyResult<&'py PyString> {
    let codec = get_codec(encoding)?;

    let out = match errors {
        "strict" => codec
            .decode_without_bom_handling_and_without_replacement(input_bytes)
            .ok_or(PyValueError::new_err("Cannot decode"))?,

        _ => {
            return Err(PyLookupError::new_err(format!(
                "unknown error handler name '{}'",
                errors
            )))
        }
    };

    Ok(PyString::new(py, out.as_ref()))
}

#[pymodule]
fn endec(_py: Python, module: &PyModule) -> PyResult<()> {
    module.add_function(wrap_pyfunction!(encode, module)?)?;
    module.add_function(wrap_pyfunction!(decode, module)?)?;
    Ok(())
}
