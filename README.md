# Beautify - JSON Formatter CLI

A simple command-line tool written in Rust that takes a JSON string as input and prints it in a beautifully formatted (pretty-printed) way.

## Features

- Parse JSON strings from command-line arguments
- Pretty-print JSON with proper indentation
- Error handling for invalid JSON
- Cross-platform support (Linux, macOS, Windows)
- Continuous Integration with GitHub Actions

## Installation

### Download pre-built binaries

Pre-built binaries for Linux, macOS, and Windows are available as artifacts from GitHub Actions:

1. Go to the [Actions tab](https://github.com/hansjlachmann/beautify/actions)
2. Click on the latest successful workflow run
3. Download the artifact for your platform:
   - `beautify-Linux-X64` for Linux
   - `beautify-macOS-ARM64` or `beautify-macOS-X64` for macOS
   - `beautify-Windows-X64` for Windows

Artifacts are retained for 7 days.

### Building from source

```bash
cargo build --release
```

The binary will be available at `./target/release/beautify`

## Usage

```bash
beautify '<json-string>'
```

### Examples

Simple JSON:
```bash
beautify '{"name":"John","age":30,"city":"New York"}'
```

Output:
```json
{
  "age": 30,
  "city": "New York",
  "name": "John"
}
```

Nested JSON with arrays:
```bash
beautify '{"users":[{"name":"Alice","age":25},{"name":"Bob","age":30}],"total":2}'
```

Output:
```json
{
  "total": 2,
  "users": [
    {
      "age": 25,
      "name": "Alice"
    },
    {
      "age": 30,
      "name": "Bob"
    }
  ]
}
```

## Dependencies

- `serde_json` - For JSON parsing and serialization

## CI/CD

This project uses GitHub Actions for continuous integration and testing. The workflow is split into two jobs:

**Build Job:**
- Builds the project on Linux, macOS, and Windows
- Builds both debug and release binaries
- Checks code formatting with `cargo fmt`
- Runs linting with `cargo clippy`
- Uploads release binaries as downloadable artifacts

**Test Job:**
- Runs all unit tests with `cargo test`
- Builds release binaries for integration testing
- Tests the CLI with various JSON inputs to ensure proper functionality
- Uploads tested binaries as downloadable artifacts

The workflow is triggered on pushes to `main`, `master`, and `claude/*` branches, as well as on pull requests. Build artifacts are available for download for 7 days.

## License

This project is open source.
