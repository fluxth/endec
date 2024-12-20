pub(crate) mod exceptions;

use encoding_rs::Encoding;
use pyo3::{
    prelude::*,
    types::{PyBytes, PyString},
};

use crate::{
    decode::{DecodeBOMHandler, DecodeError, DecodeErrorHandler},
    encode::{EncodeError, EncodeErrorHandler},
};

/// Web-compatible encoding and decoding library
#[pymodule]
fn _endec(_py: Python, module: &Bound<'_, PyModule>) -> PyResult<()> {
    module.add_function(wrap_pyfunction!(encode, module)?)?;
    module.add_function(wrap_pyfunction!(decode, module)?)?;
    Ok(())
}

#[pyfunction]
#[pyo3(signature = (input_str, /, encoding = "utf-8", errors = "strict"))]
pub(crate) fn encode<'py>(
    py: Python<'py>,
    input_str: &'py str,
    encoding: &'py str,
    errors: &'py str,
) -> PyResult<Bound<'py, PyBytes>> {
    let codec = Encoding::for_label(encoding.as_bytes())
        .ok_or(exceptions::encoding_lookup_failed(encoding))?;

    let error_handler = match errors {
        "strict" => EncodeErrorHandler::Strict,
        "xmlcharrefreplace" => EncodeErrorHandler::XMLCharacterReferenceReplace,
        _ => EncodeErrorHandler::InvalidHandler,
    };

    match crate::encode::encode(input_str, codec, error_handler) {
        Ok(encoded_bytes) => Ok(PyBytes::new(py, &encoded_bytes)),
        Err(error) => match error {
            EncodeError::InvalidErrorHandler => {
                Err(exceptions::error_handler_lookup_failed(errors))
            }
            EncodeError::EncodeFailed { used_encoding } => {
                Err(exceptions::encode_failed(used_encoding.name(), input_str))
            }
        },
    }
}

#[pyfunction]
#[pyo3(signature = (input_bytes, /, encoding = "utf-8", errors = "strict", *, bom = "evaluate"))]
pub(crate) fn decode<'py>(
    py: Python<'py>,
    input_bytes: &'py [u8],
    encoding: &'py str,
    errors: &'py str,
    bom: &'py str,
) -> PyResult<Bound<'py, PyString>> {
    let codec = Encoding::for_label(encoding.as_bytes())
        .ok_or(exceptions::encoding_lookup_failed(encoding))?;

    let error_handler = match errors {
        "strict" => DecodeErrorHandler::Strict,
        "replace" => DecodeErrorHandler::Replace,
        _ => DecodeErrorHandler::InvalidHandler,
    };

    let bom_handler = match bom {
        "evaluate" => DecodeBOMHandler::Evaluate,
        "evaluateall" => DecodeBOMHandler::EvaluateAll,
        "strip" => DecodeBOMHandler::Strip,
        "ignore" => DecodeBOMHandler::Ignore,
        _ => DecodeBOMHandler::InvalidHandler,
    };

    match crate::decode::decode(input_bytes, codec, error_handler, bom_handler) {
        Ok(decoded_string) => Ok(PyString::new(py, &decoded_string)),
        Err(error) => match error {
            DecodeError::DecodeFailed { used_encoding } => {
                Err(exceptions::decode_failed(used_encoding.name(), input_bytes))
            }
            DecodeError::InvalidErrorHandler => {
                Err(exceptions::error_handler_lookup_failed(errors))
            }
            DecodeError::InvalidBOMHandler => Err(exceptions::bom_handler_lookup_failed(bom)),
        },
    }
}
