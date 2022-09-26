use pyo3::prelude::*;

mod app;
mod interface;
mod renderer;
mod tools;

#[pyfunction]
fn show_window(width: u32, height: u32) -> PyResult<()> {
    app::create_window(width, height);
    Ok(())
}

/// A Python module implemented in Rust.
#[pymodule]
fn mesh_tools(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(show_window, m)?)?;
    Ok(())
}
