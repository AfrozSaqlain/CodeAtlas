# CodeAtlas

CodeAtlas is a personal learning repository for programming basics. Instead of being only a collection of files, each language folder is meant to work like a small tutorial with simple examples you can compile or run.

## What You Can Learn Here

## <img src="https://cdn.jsdelivr.net/gh/devicons/devicon/icons/c/c-original.svg" width="24"/>

C is a compiled language used for learning low-level programming concepts such as functions, memory, variables, and program structure.

Compile and run:

```bash
gcc -o hello hello.c
./hello
```

## <img src="https://cdn.jsdelivr.net/gh/devicons/devicon/icons/cplusplus/cplusplus-original.svg" width="24"/>

C++ extends C with stronger abstractions, object-oriented programming support, and a large standard library. It is commonly used for systems software, games, desktop applications, and performance-sensitive programs.

Compile and run:

```bash
g++ -o app app.cpp
./app
```

## <img src="https://cdn.jsdelivr.net/gh/devicons/devicon/icons/lua/lua-original.svg" width="24"/> Lua

Lua is a lightweight scripting language often used in games, plugins, and embedded systems. It is simple to start with because variables do not need explicit type declarations.

Run:

```bash
lua intro.lua
```

## <img src="https://cdn.jsdelivr.net/gh/devicons/devicon/icons/rust/rust-original.svg" width="24"/> Rust

Rust is a compiled language focused on speed and memory safety. Its ownership and borrowing system helps prevent many common bugs at compile time.

Run with Cargo:

```bash
cargo run
```

## <img src="https://cdn.jsdelivr.net/gh/devicons/devicon/icons/python/python-original.svg" width="24"/> Python

Python is a high-level interpreted language known for readability and fast development. This repository uses it for intermediate concepts such as decorators, dunder methods, classes, and packaged command-line tools.

Run:

```bash
python script.py
```

## <img src="https://cdn.jsdelivr.net/gh/devicons/devicon/icons/docker/docker-original.svg" width="24"/> Docker

Docker is not a programming language, but it is an important developer tool. It packages an application and its dependencies into a container so it can run consistently across machines.

Basic workflow:

```bash
docker buildx build -t my-app .
docker run --rm my-app
```

## Repository Layout

```text
CodeAtlas/
|-- C/
|-- CPP/
|-- Docker/
|-- Lua/
|-- Python/
`-- Rust/
```

Open a folder's README for a more focused beginner tutorial.
