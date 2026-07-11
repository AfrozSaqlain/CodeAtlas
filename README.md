# CodeAtlas

CodeAtlas is a personal learning repository with small, focused examples across several programming and developer-tooling topics. The current repository covers Python, Rust, C/C++, Lua, and Docker.

## Repository Structure

```text
CodeAtlas/
|-- C_Lang/                 # C and C++ compile/run examples
|-- Docker/                 # Docker notes and a packaged Python CLI demo
|   `-- test-cli/
|-- Lua/                    # Lua syntax and game-development notes
|   `-- Game/
|-- Python/                 # Intermediate Python concepts
|   |-- Decorators/
|   |-- Dunder/
|   `-- OOP/
`-- Rust/                   # Cargo projects for Rust basics and data structures
    |-- Basics/
    |-- Data_Structures/
    `-- projects/
```

## Contents

### Python

The Python section contains examples for intermediate language features:

- `Python/Decorators/`: `@property`, `@staticmethod`, `@classmethod`, `dataclass`, `functools`, and timing decorators.
- `Python/Dunder/`: examples for Python special methods such as `__str__`, `__repr__`, and operator overloading.
- `Python/OOP/`: notebook-based notes for classes and object-oriented programming.

Run a script from the repository root with:

```bash
python Python/Decorators/timer.py
python Python/Dunder/add_dunder.py
```

### Rust

The Rust section contains multiple independent Cargo projects:

- `Rust/Basics/`: basic syntax, variables, ownership, borrowing, loops, functions, matching, strings, and related topics.
- `Rust/Data_Structures/`: arrays, vectors, and data-structure practice.
- `Rust/projects/`: a separate Rust project sandbox.

Run a main project:

```bash
cd Rust/Basics
cargo run
```

Run a specific Rust exercise from `src/bin/`:

```bash
cd Rust/Basics
cargo run --bin variables
```

### C and C++

`C_Lang/` contains simple C and C++ source files:

- `hello.c`
- `math_op.cpp`

Compile and run them with:

```bash
gcc -o hello C_Lang/hello.c
./hello

g++ -o math_op C_Lang/math_op.cpp
./math_op
```

### Lua

`Lua/` contains introductory Lua examples and a small game-oriented folder:

- `intro.lua`
- `Game/game.lua`

Run Lua scripts with:

```bash
lua Lua/intro.lua
lua Lua/Game/game.lua
```

### Docker

`Docker/README.md` is a Docker quick reference covering installation, Buildx, images, containers, volumes, and common commands.

`Docker/test-cli/` packages a small Python CLI named `plot-maths`. The CLI plots a simple quadratic curve and writes `Plot.jpg`.

Build and run the image:

```bash
cd Docker/test-cli
docker buildx build -t plot-maths .
docker run --rm -it -v "$PWD:/workspace" plot-maths
```

Inside the container, run:

```bash
cd /workspace
plot-maths
```

## Requirements

Install the tools needed for the folders you want to use:

- Python 3.8+ for the Python scripts and the Docker CLI package.
- Rust toolchain with Cargo for `Rust/`.
- `gcc` and `g++` for `C_Lang/`.
- Lua for `Lua/`.
- Docker with Buildx for `Docker/`.

## Getting Started

Clone the repository and move into it:

```bash
git clone https://github.com/AfrozSaqlain/CodeAtlas.git
cd CodeAtlas
```

Then choose a topic folder and follow its local README or the commands above.
