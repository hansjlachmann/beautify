# Beautify - JSON Formatter CLI

A simple command-line tool written in Rust that takes a JSON string as input and prints it in a beautifully formatted (pretty-printed) way.

## Features

- Parse JSON strings from command-line arguments
- Pretty-print JSON with proper indentation
- Error handling for invalid JSON

## Installation

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

## License

This project is open source.
