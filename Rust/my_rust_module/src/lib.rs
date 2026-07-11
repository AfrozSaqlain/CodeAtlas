use pyo3::prelude::*;

#[pymodule]
fn my_rust_module(m: &Bound<'_, PyModule>) -> PyResult<()> {
    #[pyfunction]
    fn fib(n: usize) -> u64 {
        let (mut a, mut b): (u64, u64) = (0, 1);
        for _ in 0..n {
            (a, b) = (b, a + b);
        }
        a
    }

    m.add_function(wrap_pyfunction!(fib, m)?)?;
    Ok(())
}