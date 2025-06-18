# Chunker

A simple and efficient Rust utility to split slices into fixed-size chunks, returning owned `Vec<T>` chunks.

---

## 📦 Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
chunker = "0.1.0"
```

> Replace `"0.1.0"` with the latest version from [crates.io](https://crates.io/crates/chunker)

---

## 🚀 Usage

```rust
use chunker::chunk;

fn main() {
    let data = vec![1, 2, 3, 4, 5];
    let result = chunk(&data, 2).unwrap();
    assert_eq!(result, vec![vec![1, 2], vec![3, 4], vec![5]]);
}
```

---

## 🧩 Function Behavior

```rust
pub fn chunk<T: Clone>(array: &[T], chunk_size: usize) -> Result<Vec<Vec<T>>, &'static str>
```

The `chunk` function behaves as follows:

- ✅ Returns an error if `chunk_size == 0`
- ✅ Returns an empty vector if the input slice is empty
- ✅ Returns a single chunk containing all elements if `chunk_size >= array.len()`
- ✅ Returns multiple chunks of up to `chunk_size` elements otherwise

Each chunk is a cloned `Vec<T>`, preserving data integrity and independence.

---

## 🧪 Test Coverage

The implementation is covered by comprehensive unit tests, including:

- ✅ Primitive types: `i32`, `bool`, etc.
- ✅ Strings and owned `String`
- ✅ Structs (`Clone + PartialEq`)
- ✅ Enums (e.g., `Status::Ok`, `Status::Error`)
- ✅ Nested collections (e.g., `Vec<Vec<T>>`)
- ✅ Edge cases:
    - Empty input
    - `chunk_size == 0` (returns error)
    - `chunk_size >= input.len()` (returns single chunk)

To run tests:

```bash
cargo test
```

---

## 🔒 Safety

- 100% safe Rust (`#![forbid(unsafe_code)]`)
- No `unsafe` blocks used
- Pure functional logic

---

## 📄 License

This project is dual-licensed under either:

- [Apache 2.0](LICENSE-APACHE)

You may freely choose either license.

---

## 🤝 Contributing

Contributions, bug reports, and feature requests are welcome.  
Please open an issue or submit a pull request.

---
