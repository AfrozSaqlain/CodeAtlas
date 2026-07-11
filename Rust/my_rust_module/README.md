# My Rust-Python Extension

A high-performance Python package powered by a Rust backend using **PyO3**, built and managed entirely within a **Conda** environment.

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
python -c "import my_rust_module; print(my_rust_module.sum_as_string(10, 20))"
```

### Example Python Script

```python
import my_rust_module

result = my_rust_module.sum_as_string(5, 7)

print(f"Result from Rust: {result}")
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
>>> my_rust_module.sum_as_string(2, 3)
'5'
```
