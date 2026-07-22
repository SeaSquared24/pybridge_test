//! This is the main library file for the Rust backend. 
//! It defines the functions that will be exposed to Python using PyO3.
//! The stack is as follows: Python (Tkinter) -> Rust (PyO3) -> Rust (Backend Logic).
//! PyO3 can be split into two parts: pyfunction and pymodule. 
//! The pyfunction exposes the Rust function to Python, while the pymodule defines the Python module that will be built.

use pyo3::prelude::*;

/// pyfunction exposes the Rust function to Python.
/// The name parameter allows you to specify the name of the function as it will be called from Python.
/// This particular function refers to another function defined in the same file, which is the actual logic of the function.
/// Depending on the case, you may leave the function logic in pyfunction 
/// or refer to another function defined in the same file.
#[pyfunction(name = "button_pressed")]
fn py_button_pressed() -> String {
    button_pressed()
}

pub fn button_pressed() -> String {
    "Hello from Rust!".to_string()
}

/// py_greet is another example of a pyfunction that exposes a Rust function to Python.
#[pyfunction(name = "greet")]
fn py_greet(name: &str) -> String {
    greet(name)
}

pub fn greet(name: &str) -> String {
    format!("Hello, {}!", name)
}

/// The module definition builds the Python module.
#[pymodule]
fn rust_backend(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(py_button_pressed, m)?)?;
    // You can add more functions here if needed
    m.add_function(wrap_pyfunction!(py_greet, m)?)?;
    Ok(())
}