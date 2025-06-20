# Pencil-Box

A simple and efficient Rust utility for:

- 📦 Splitting slices into fixed-size chunks with `chunk`
- 🔁 Removing duplicate elements from vectors with `uniq` and `uniq_performant`

---

## 📦 Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
pencil-box = "0.1.2"
```

> Replace `"0.1.2"` with the latest version from [crates.io](https://crates.io/crates/pencil-box)

---

## 🚀 Usage

### ✂️ Chunking Slices

```rust
use pencil_box::chunk;

fn main() {
    let data = vec![1, 2, 3, 4, 5];
    let result = chunk(&data, 2).unwrap();
    assert_eq!(result, vec![vec![1, 2], vec![3, 4], vec![5]]);
}
```

### 🧹 Deduplicating Vectors

```rust
use pencil_box::{uniq, uniq_performant};

fn main() {
    let mut items = vec![1, 2, 2, 3, 1];
    uniq(&mut items); // or uniq_performant(&mut items);
    assert_eq!(items, vec![1, 2, 3]);
}
```

---

## 🧩 Function Behavior

### `chunk`

```rust
pub fn chunk<T: Clone>(array: &[T], chunk_size: usize) -> Result<Vec<Vec<T>>, &'static str>
```

- ✅ Returns an error if `chunk_size == 0`
- ✅ Returns an empty vector if the input slice is empty
- ✅ Returns a single chunk if `chunk_size >= array.len()`
- ✅ Returns multiple chunks of up to `chunk_size` elements otherwise

Each chunk is cloned into an owned `Vec<T>`.

---

### `uniq`

```rust
pub fn uniq<T: Eq + Hash + Clone>(values: &mut Vec<T>)
```

- ✅ Uses the standard `HashSet` to remove duplicates in-place
- ✅ Retains the first occurrence of each element
- ✅ Preserves relative ordering
- 🔁 Best for general-purpose use

---

### `uniq_performant`

```rust
pub fn uniq_performant<T: Eq + Hash + Clone>(values: &mut Vec<T>)
```

- 🚀 Uses `AHashSet` for faster hashing and performance
- ✅ Retains the first occurrence of each element
- ✅ Preserves relative ordering
- ⚡ Ideal for high-throughput or performance-critical use cases

---

## 🔒 Safety

- 100% safe Rust (`#![forbid(unsafe_code)]`)
- No `unsafe` blocks used
- Pure functional logic

---

## 📄 License

This project is dual-licensed under either:

- [MIT](LICENSE-MIT) OR
- [Apache 2.0](LICENSE-APACHE)

You may freely choose either license.

---

## 🤝 Contributing

Contributions, bug reports, and feature requests are welcome.  
Please open an issue or submit a pull request.

---

## 🌐 Links

- [Crates.io](https://crates.io/crates/pencil-box)
- [Repository](https://github.com/rocketnozzle/pencil-box)
