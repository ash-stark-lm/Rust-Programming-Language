---

# Getting Started with Rust Projects

## Running a single Rust file

1. Create a Rust source file, e.g., `name.rs`.

2. Compile the file using the Rust compiler (`rustc`):

```bash
rustc name.rs
```

3. Run the compiled executable:

* On **Windows**:

```bash
.\name.exe
```

* On **Linux/macOS**:

```bash
./name
```

---

## Understanding the `main` function

- The `main` function is special in Rust.
- It’s the entry point for every executable Rust program.
- Similar to `int main()` in C/C++.

Example:

```rust
fn main() {
    println!("Hello, world!");
}
```

---

## Creating and managing projects with Cargo

- Cargo is Rust’s official package manager and build tool.
- Check if Cargo is installed:

```bash
cargo --version
```

- Create a new project named `name`:

```bash
cargo new name
```

This creates a new directory called `name` with the standard Rust project structure.

- Navigate into the project directory:

```bash
cd name
```

- Build the project:

```bash
cargo build
```

The compiled executable will be placed in:

```bash
./target/debug/name.exe   # Windows
./target/debug/name      # Linux/macOS
```

- Alternatively, to compile and run in one step:

```bash
cargo run
```

---

This setup will help you start writing, compiling, and running Rust programs smoothly.

---
