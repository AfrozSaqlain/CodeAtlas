# 🦀 Rust Quick Start Guide

This project serves as a sandbox for learning Rust basics, running standalone scripts, and managing Cargo workflows.

---

## 🚀 How to Run the Main Project

To compile and run the main entry point (`src/main.rs`), navigate to this project folder in your terminal and execute:

```bash
cargo run
```

### ⚡ Shortcuts & Tips
* **Skip typing `cargo run` every time:** You can install `cargo-watch` to automatically re-run your code every time you press save:
  ```bash
  cargo install cargo-watch
  cargo watch -x run
  ```
* **Run the compiled binary directly:** If you don't want to use Cargo, you can run the generated executable from your terminal:
  * **Mac/Linux:** `./target/debug/my_project`
  * **Windows:** `.\target\debug\my_project.exe`

---

## 📂 Project Structure & How to Add Code

Do not drop random `.rs` files directly into the `src/` directory. Instead, choose one of the two structures below depending on what you are building:

### Option A: Running Multiple Independent Scripts (Recommended for Learning)
If you want to create separate, unrelated exercises or test scripts, place them inside the `src/bin/` directory.

**Structure:**
```text
my_project/
├── Cargo.toml
└── src/
    ├── main.rs         # Runs with: cargo run
    └── bin/            # Create this folder for individual scripts
        ├── exercise1.rs
        └── exercise2.rs
```

**How to run a specific script:**
```bash
cargo run --bin exercise1
```

---

### Option B: Splitting a Large App into Modules
If you are building one single application and want to break `main.rs` into smaller, organized files:

1. Create your new file directly in `src/` (e.g., `src/tools.rs`).
2. Register the module at the very top of your `src/main.rs` file:
   ```rust
   mod tools; 

   fn main() {
       tools::your_function(); 
   }
   ```
3. Run the entire application normally using `cargo run`.

---

## 🛠️ Creating a Completely New Project
When you are ready to move away from this sandbox and start a brand-new standalone application, run:

```bash
cargo new your_new_project_name
cd your_new_project_name
```

