use std::borrow::Cow;

use pyo3::{
    exceptions::{PyLookupError, PyValueError},
    prelude::*,
    types::{PyBytes, PyString},
};

mod exceptions {
    pyo3::import_exception!(endec.exceptions, EncodeError);
    pyo3::import_exception!(endec.exceptions, DecodeError);
}

fn get_codec(name: &str) -> PyResult<&'static encoding_rs::Encoding> {
    encoding_rs::Encoding::for_label(name.as_bytes()).ok_or(PyLookupError::new_err(format!(
        "unknown encoding: {}",
        name
    )))
}

#[pyfunction]
#[pyo3(signature = (input_str, /, encoding = "utf-8", errors = "strict"))]
fn encode(py: Python, input_str: &str, encoding: &str, errors: &str) -> PyResult<PyObject> {
    let (out, used_encoding, has_unmappable) = get_codec(encoding)?.encode(input_str);
    let convert_to_pybytes = || PyBytes::new(py, &out).into();

    if !has_unmappable {
        return Ok(convert_to_pybytes());
    }

    match errors {
        "strict" => Err(exceptions::EncodeError::new_err((
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

#[pyfunction]
#[pyo3(signature = (input_bytes, /, encoding = "utf-8", errors = "strict", bom = "evaluate"))]
fn decode<'py>(
    py: Python<'py>,
    input_bytes: &'py [u8],
    encoding: &'py str,
    errors: &'py str,
    bom: &'py str,
) -> PyResult<&'py PyString> {
    let codec = get_codec(encoding)?;

    let mut evaluated_encoding = encoding;

    let mut decode = || -> PyResult<(Cow<'py, str>, bool)> {
        match bom {
            "evaluate" => {
                let (out, used_encoding, has_replaced) = codec.decode(input_bytes);
                evaluated_encoding = used_encoding.name();
                Ok((out, has_replaced))
            }
            "strip" => Ok(codec.decode_with_bom_removal(input_bytes)),
            "replace" => Ok(codec.decode_without_bom_handling(input_bytes)),
            "ignore" => {
                let out = codec
                    .decode_without_bom_handling_and_without_replacement(input_bytes)
                    .ok_or(exceptions::DecodeError::new_err((
                        evaluated_encoding.to_owned(),
                        input_bytes.to_vec(),
                    )))?;

                Ok((out, false))
            }
            _ => Err(PyLookupError::new_err(format!(
                "unknown byte-order mark handler name: {}",
                bom
            ))),
        }
    };

    let out = match errors {
        "strict" => {
            let (out, has_replaced) = decode()?;

            if has_replaced {
                // has replaced characters but strict mode was used
                // TODO add reason in exception
                return Err(exceptions::DecodeError::new_err((
                    evaluated_encoding.to_owned(),
                    input_bytes.to_owned(),
                )));
            }

            out
        }
        "replace" => decode()?.0,
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
