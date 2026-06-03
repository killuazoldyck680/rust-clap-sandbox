# Enterprise Async CLI Tool (Rust Sandbox)

A high-performance, production-grade command-line interface (CLI) built in Rust. This project evolved from a basic CLI into a fully asynchronous automation tool that manages local configuration files, handles system environment variables, communicates over HTTPS with external web APIs, and provides interactive terminal feedback.

## 🚀 Features

- **Multi-Layered Configuration**: Merges terminal flags, system environment variables (`API_KEY`), and local `config.json` configuration files with strict hierarchy parsing.
- **Asynchronous Architecture**: Network calls run natively on the `tokio` async runtime for non-blocking I/O operations.
- **Type-Safe API Integration**: Uses `reqwest` and `serde` to seamlessly deserialize live JSON payloads from remote servers over secure HTTPS.
- **Production-Grade Error Handling**: Utilizes `anyhow` for idiomatic error propagation and structural debugging context instead of unsafe panics or unwraps.
- **Interactive UX/UI**: Implements real-time animated terminal loading spinners via `indicatif` to mask network latency.

---

## 🛠️ Installation & Setup

Ensure you have Rust and Cargo installed.

1. Clone the repository:
   git clone https://github.com/killuazoldyck680/rust-clap-sandbox.git
   cd rust-clap-sandbox

2. Create a local configuration file named `config.json` in the root directory:
   {
     "api_key": "your_fallback_api_key_here",
     "default_format": "json"
   }

3. Build the project:
   cargo build --release

---

## 💻 Usage Examples

### 1. Base Command (Configuration Check)
Running the app without a subcommand verifies your configuration loading layer:
   cargo run

### 2. Live API Network Request (Ping Subcommand)
Trigger a live, async network call to test secure header injection and JSON deserialization:
   cargo run -- ping

### 3. Environment Variable Override
Verify that system environment variables take higher priority than your local JSON configuration:
   API_KEY=my_secret_env_override cargo run -- ping

---

## 🧠 Production Concepts Mastered

- **Advanced Clap Attributes**: Implemented `env = "VARIABLE"` macros and custom `value_parser` layers to validate OS path structures before `main` runs.
- **Serde Field Mapping**: Handled casing variations between server responses and local data structs using the `#[serde(rename = "...")]` attribute.
- **Idiomatic Error Flow**: Mastered the `?` operator alongside `.context()` attachments to build clean, descriptive error tracks for end users.
- **Terminal State Management**: Learned to safely spin up, refresh, and clear graphical terminal components without corrupting stdout streams.