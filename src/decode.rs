use std::borrow::Cow;

use encoding_rs::{UTF_16BE, UTF_16LE, UTF_8};
use pyo3::exceptions::PyLookupError;
use pyo3::prelude::*;
use pyo3::types::PyString;

use crate::exceptions::DecodeError;
use crate::utils::get_encoding_by_name;

#[pyfunction]
#[pyo3(signature = (input_bytes, /, encoding = "utf-8", errors = "strict", *, bom = "evaluate"))]
pub(crate) fn decode<'py>(
    py: Python<'py>,
    input_bytes: &'py [u8],
    encoding: &'py str,
    errors: &'py str,
    bom: &'py str,
) -> PyResult<Bound<'py, PyString>> {
    let codec = get_encoding_by_name(encoding)?;

    let mut evaluated_encoding = codec.name();

    let mut decode = || -> PyResult<(Cow<'py, str>, bool)> {
        match bom {
            "evaluate" => {
                let (out, used_encoding, has_replaced) =
                    if codec == UTF_8 || codec == UTF_16BE || codec == UTF_16LE {
                        codec.decode(input_bytes)
                    } else {
                        let (out, has_replaced) = codec.decode_with_bom_removal(input_bytes);
                        (out, codec, has_replaced)
                    };

                evaluated_encoding = used_encoding.name();
                Ok((out, has_replaced))
            }
            "evaluateall" => {
                let (out, used_encoding, has_replaced) = codec.decode(input_bytes);
                evaluated_encoding = used_encoding.name();
                Ok((out, has_replaced))
            }
            "strip" => Ok(codec.decode_with_bom_removal(input_bytes)),
            "ignore" => {
                if errors == "strict" {
                    let out = codec
                        .decode_without_bom_handling_and_without_replacement(input_bytes)
                        .ok_or(DecodeError::new_err((
                            evaluated_encoding.to_owned(),
                            input_bytes.to_vec(),
                        )))?;

                    Ok((out, false))
                } else {
                    // replace
                    Ok(codec.decode_without_bom_handling(input_bytes))
                }
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
                return Err(DecodeError::new_err((
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

    Ok(PyString::new_bound(py, out.as_ref()))
}
