

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
