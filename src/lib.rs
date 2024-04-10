mod decode;
mod encode;
mod utils;

mod exceptions {
    pyo3::import_exception!(endec.exceptions, EncodeError);
    pyo3::import_exception!(endec.exceptions, DecodeError);
}

use pyo3::prelude::*;

#[pymodule]
fn _endec(_py: Python, module: &Bound<'_, PyModule>) -> PyResult<()> {
    module.add_function(wrap_pyfunction!(encode::encode, module)?)?;
    module.add_function(wrap_pyfunction!(decode::decode, module)?)?;
    Ok(())
}
