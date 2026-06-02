# Rust Clap Sandbox

A lightweight, clean, and idiomatic command-line interface (CLI) built in Rust using the `clap` (v4) derive API. This project serves as a practical sandbox for mastering CLI argument parsing, subcommands, and navigating Rust's strict ownership and borrowing rules.

## 🚀 Features

- **Global Arguments**: Accept a `--name` and an optional loop `--count` (defaults to 1).
- **Optional Subcommands**: Includes `add` and `list` subcommands.
- **Robust Memory Management**: Implements safe reference matching (`&`) to prevent partial-move compiler errors.
- **Auto-Generated Help**: Leverages `clap`'s built-in reflection to generate clean terminal documentation.

---

## 🛠️ Installation & Setup

Ensure you have Rust and Cargo installed.

1. Clone the repository:
   git clone https://github.com/killuazoldyck680/rust-clap-sandbox.git
   cd rust-clap-sandbox

2. Build the project:
   cargo build --release

---

## 💻 Usage Examples

You can run the CLI directly using `cargo run --`.

### 1. Base Command (Greeting)
   cargo run -- --name "Alice" --count 3

Output:
   Hello Alice
   Hello Alice
   Hello Alice

### 2. Using Subcommands
Add an item:
   cargo run -- add "Rust Learning Journey"

Output: 
   Adding: Rust Learning Journey

List items (with verbose flag):
   cargo run -- list --verbose

Output: 
   Listing all items verbosely...

### 3. View Auto-Generated Help
   cargo run -- --help
   # Or check help for a specific subcommand:
   cargo run -- add --help

---

## 🧠 What I Learned Building This

- **Clap Derive Macro**: How to structure structs and enums with `#[derive(Parser)]` and `#[derive(Subcommand)]`.
- **Rust Ownership & Matching**: How matching on multi-layered `Option` types causes "partial moves," and how to correctly use references (`&cli.name`) to keep the data container intact.
- **Dereferencing**: Utilizing the `*` operator to unpack borrowed primitive types (like `&bool`) for control flow logic.