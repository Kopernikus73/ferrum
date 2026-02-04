mod evaluation_engine;

use pyo3::prelude::*;

/// A Python module implemented in Rust.
#[pymodule]
mod chess_bot {
    use pyo3::prelude::*;
    use super::evaluation_engine;

    /// Formats the sum of two numbers as string.
    #[pyfunction]
    fn sum_as_string(a: usize, b: usize) -> PyResult<String> {
        Ok((a + b).to_string())
    }

    #[pyfunction]
    fn eval(chess_move: u32, field: Vec<u32>) -> PyResult<u32> {
        Ok(evaluation_engine::eval(chess_move, field))
    }
}


