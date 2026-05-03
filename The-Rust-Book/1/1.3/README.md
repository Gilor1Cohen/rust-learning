# Cargo

Cargo is Rust’s build system and package manager.

It helps you create, build, run, test, and manage Rust projects.

### Creating a Project with Cargo

Create a new Rust project:

```bash
cargo new <project_name>
```

Move into the project folder:

```bash
cd <project_name>
```

### Building and Running a Cargo Project

Build the project:

```bash
cargo build
```

Run the project:

```bash
cargo run
```

### Cargo Project Structure

#### `Cargo.toml`

`Cargo.toml` is the main configuration file of the project.

It tells Cargo important information about the project, such as:

- project name
- project version
- Rust edition
- dependencies

Example:

```toml
[package]
name = "hello_rust"
version = "0.1.0"
edition = "2021"

[dependencies]
```

#### `src/`

The `src` folder contains the source code of the project.

Source code means the Rust files that you write.

In a basic Rust program, the main file is:

```txt
src/main.rs
```
