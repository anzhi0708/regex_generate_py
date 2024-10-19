use pyo3::prelude::*;
use rand;
use regex_generate::{Generator, DEFAULT_MAX_REPEAT};

#[pyfunction]
fn generate(pattern: &str) -> PyResult<String> {
    let mut gen = Generator::new(pattern, rand::thread_rng(), DEFAULT_MAX_REPEAT).unwrap();
    let mut buffer = vec![];
    gen.generate(&mut buffer).unwrap();
    let output = String::from_utf8(buffer).unwrap();
    Ok(output)
}

/// A Python module implemented in Rust.
#[pymodule]
fn regex_generate_py(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(generate, m)?)?;
    Ok(())
}
