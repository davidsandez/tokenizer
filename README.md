# Tokenizer

Lightweight CLI written in Rust for replacing JSON and TOML values with unique UUID-based tokens.

Tokenizer can operate in two modes:

- Generate a single UUID token directly from the command line.
- Process a JSON or TOML file and replace its values with unique tokens.

---

## 🚀 What does it do?

Tokenizer generates UUID v4 tokens and can use them to replace values in JSON and TOML documents.

Given an input such as:

```json
{
  "name": "John",
  "age": 30,
  "active": true,
  "roles": ["admin", "user"]
}

Tokenizer replaces the leaf values with unique tokens:

{

  "name": "550e8400-e29b-41d4-a716-446655440000",

  "age": "7c9e6679-7425-40de-944b-e07fc1f90ae7",

  "active": "16fd2706-8baf-433b-82eb-8c7fada847da",

  "roles": [

    "f47ac10b-58cc-4372-a567-0e02b2c3d479",

    "6ba7b810-9dad-11d1-80b4-00c04fd430c8"

  ]

}

Each replaced value receives a unique token.

---

## ⚙️ Features

- Generate UUID v4 tokens.
- Process JSON files.
- Process TOML files.
- Recursively traverse nested objects and arrays.
- Generate unique tokens within each processed document.
- Preserve the structure of the input document.
- Output the transformed document to standard output.
- Fail with an error when the input file cannot be read, parsed, or is not a supported file type.

---

## 📦 Installation

### Requirements

- Rust and Cargo.

### Build from source

Clone the repository:

git clone https://github.com/davidsandez/tokenizer.git

cd tokenizer

Build the release binary:

cargo build --release

The executable will be available at:

target/release/tokenizer

You can also install it globally with:

cargo install --path .

---

## ▶️ Usage

### Generate a single token

Running Tokenizer without an input file generates a single UUID v4 token:

tokenizer

Example:

550e8400-e29b-41d4-a716-446655440000

### Process a JSON file

tokenizer --input config.json

or:

tokenizer -i config.json

The transformed JSON is written to standard output.

### Process a TOML file

tokenizer --input config.toml

or:

tokenizer -i config.toml

The transformed TOML is written to standard output.

---

## 📄 Input handling

Tokenizer currently supports files with the following extensions:

|Extension|Supported|
|---|---|
|`.json`|Yes|
|`.toml`|Yes|
|Other extensions|No|

The file extension determines which parser is used.

Unsupported extensions produce an error:

Error: Unsupported file type (only .json or .toml allowed).

---

## 🔄 Transformation behavior

Tokenizer recursively traverses JSON and TOML structures.

Objects and tables are preserved, while leaf values are replaced with generated tokens.

For example:

{

  "database": {

    "host": "localhost",

    "port": 5432

  }

}

becomes structurally equivalent to:

{

  "database": {

    "host": "<generated-token>",

    "port": "<generated-token>"

  }

}

Arrays are traversed recursively as well:

{

  "users": [

    "alice",

    "bob",

    "charlie"

  ]

}

becomes:

{

  "users": [

    "<generated-token>",

    "<generated-token>",

    "<generated-token>"

  ]

}

Tokens are generated using UUID version 4.

---

## 🔒 Limitations

Tokenizer is intentionally simple.

Currently:

- Only JSON and TOML input files are supported.
- The output is written to standard output.
- Input files are not modified in place.
- All leaf values are replaced with strings.
- Object keys are not replaced.
- JSON and TOML comments are not preserved as part of the transformed output.
- Token generation is random and does not provide deterministic output.

Tokenizer does not encrypt, hash, or securely anonymize values. The generated UUIDs are replacement tokens, not reversible or cryptographic representations of the original values.

---

## 🛠️ Development

Check the project:

cargo check

Format the code:

cargo fmt

Verify formatting without modifying files:

cargo fmt --check

Run Clippy:

cargo clippy --all-targets --all-features -- -D warnings

Run the test suite:

cargo test

Build the release binary:

cargo build --release

---

## 📌 Philosophy

Tokenizer is designed around a few simple principles:

- **Minimal** — focused on a small and well-defined task.
- **Lightweight** — distributed as a single Rust binary.
- **Predictable** — the input structure is preserved while leaf values are replaced.
- **Composable** — transformed output can be redirected or consumed by other command-line tools.
- **Easy to distribute** — no runtime infrastructure is required.