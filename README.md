# URL Encoder CLI

A simple command-line tool written in Rust that encodes input strings using RFC
3986 URL Path and Query encoding. This utility helps you properly encode text
for use in URLs, handling special characters and spaces.

Copyright 2025 Axiom Data Science, LLC.

## Features

- Encode entire input strings using RFC 3986 standards
- Support for multiple input arguments
- Simple and straightforward CLI interface

## Installation

```
cargo install --path .
```

### Prerequisites

- Rust programming language (latest stable version recommended)
- Cargo package manager

### Build from Source

1. Clone the repository:
   ```bash
   git clone http://git.axiom/luke/urlencode.git
   cd urlencode
   ```

2. Build the project:
   ```bash
   cargo build --release
   ```

3. The binary will be available at `target/release/urlencode`

## Usage

```bash
urlencode <input-string>
```

### Examples

```bash
# Encode a simple string
$ urlencode "Hello World"
Hello%20World

# Encode multiple words
$ urlencode Hello World Rust
Hello%20World%20Rust

# Encode special characters
$ urlencode "https://example.com/path?param=value"
https%3A%2F%2Fexample.com%2Fpath%3Fparam%3Dvalue
```

## Dependencies

- `urlencoding`: For RFC 3986 URL encoding
- `clap`: For parsing command-line arguments

## Contributing

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/AmazingFeature`)
3. Commit your changes (`git commit -m 'Add some AmazingFeature'`)
4. Push to the branch (`git push origin feature/AmazingFeature`)
5. Open a Pull Request

## License

Distributed under the MIT License. See `LICENSE` for more information.

## Contact

Luke Campbell - luke@axds.co

Project Link: [http://git.axiom/luke/urlencode](http://git.axiom/luke/urlencode)
