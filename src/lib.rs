use pyo3::{
    prelude::*,
    types::{PyBytes, PyString},
};

fn get_codec(name: &str) -> &'static encoding_rs::Encoding {
    encoding_rs::Encoding::for_label(name.as_bytes()).unwrap()
}

#[pyfunction]
fn encode(py: Python, input_str: &str, encoding: &str) -> PyResult<PyObject> {
    let (out, _, _) = get_codec(encoding).encode(input_str);
    Ok(PyBytes::new(py, &out).into())
}

#[pyfunction]
fn decode<'py>(
    py: Python<'py>,
    input_bytes: &'py [u8],
    encoding: &'py str,
) -> PyResult<&'py PyString> {
    let (out, _, _) = get_codec(encoding).decode(input_bytes);
    Ok(PyString::new(py, out.as_ref()))
}

#[pymodule]
fn endec(_py: Python, module: &PyModule) -> PyResult<()> {
    module.add_function(wrap_pyfunction!(encode, module)?)?;
    module.add_function(wrap_pyfunction!(decode, module)?)?;
    Ok(())
}
