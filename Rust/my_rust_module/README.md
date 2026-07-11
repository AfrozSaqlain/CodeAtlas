# Rust And Python Extensions: A Basic Introduction

Rust can be used to write fast native extensions for Python. This is useful when Python code needs better performance for CPU-heavy work, while still keeping a Python-friendly API.

## Overview

Rust can be used to write fast native extensions for Python, making it possible to accelerate CPU-intensive code while maintaining a familiar Python interface.

This project uses:

- **PyO3** — exposes Rust functions and modules to Python.
- **maturin** — builds the Rust crate and installs the compiled extension into the active Python environment.

Unlike regular Python modules, Rust code is compiled into a native shared library that Python imports like any other package.

Although called from Python, the function is executed entirely in Rust.

---

## Prerequisites

Before getting started, ensure the following are installed:

- **Miniconda** or **Anaconda**
- **Rust toolchain** (installed via https://rustup.rs)

Verify that Rust is available in your shell:

```bash
cargo --version
rustc --version
```

---

## Environment Setup

To isolate dependencies and avoid conflicts (such as overlapping `VIRTUAL_ENV` and `CONDA_PREFIX` variables), all development is done inside a dedicated Conda environment.

### 1. Create a Conda Environment

```bash
conda create -n rust_python_env python=3.10 -y
```

### 2. Activate the Environment

```bash
conda activate rust_python_env
```

### 3. Install Maturin

`maturin` builds the Rust crate and installs the compiled Python extension directly into the active Conda environment.

```bash
pip install maturin
```

---

## Project Initialization

If you are starting a new project, initialize a PyO3 crate with:

```bash
maturin init
```

When prompted, choose:

```
pyo3
```

This generates a Rust crate already configured for PyO3.

---

## Development Workflow

### Debug Build

For fast compilation during development:

```bash
maturin develop
```

### Release Build

For optimized performance (recommended for benchmarking and production):

```bash
maturin develop --release
```

After the build completes, the extension is automatically installed into the active Conda environment and can be imported immediately from Python.

---

## Verification

### Quick Test

```bash
python -c "import my_rust_module; print(my_rust_module.fib(10))"
```

Expected output:

```text
55
```

### Example Python Script

```python
import my_rust_module

result = my_rust_module.fib(10)

print(f"Fibonacci(10): {result}")
print(f"Type: {type(result)}")
```

---

## Troubleshooting

### Error: Both `VIRTUAL_ENV` and `CONDA_PREFIX` are set

This occurs when a traditional Python virtual environment is activated on top of an active Conda environment.

**Fix:**

Deactivate both environments and activate only the Conda environment:

```bash
deactivate          # If a Python venv is active
conda deactivate    # Repeat if multiple Conda environments are stacked
conda activate rust_python_env
```

---

### Error: `cargo` or `rustc` command not found

If Rust is installed but not available in your shell, source the Rust environment:

```bash
source "$HOME/.cargo/env"
```

To make this permanent, add the above line to your shell configuration file (e.g. `~/.bashrc` or `~/.zshrc`).

---

## Typical Development Session

```bash
conda activate rust_python_env

# Make changes to the Rust code...

maturin develop

# or, for an optimized build
maturin develop --release

python
>>> import my_rust_module
>>> my_rust_module.fib(10)
55
```

---

## How PyO3 Exposes Rust Functions

PyO3 uses Rust attributes to define Python modules and functions.

```rust
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
```

Here:

- `#[pymodule]` defines the Python module.
- `#[pyfunction]` marks a Rust function that should be callable from Python.
- `m.add_function(...)` registers the function with the module.
