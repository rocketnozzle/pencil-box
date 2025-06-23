# Pencil-Box

A simple and efficient Rust utility for:

-   📦 Splitting slices into fixed-size chunks with `chunk`
-   🔁 Removing duplicate elements from vectors with `uniq` and `uniq_performant`
-   ➖ Computing set differences between vectors with `difference` and `difference_performant`
-   🗑️ Removing "empty" elements from vectors with `compact`

---

## 📦 Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
pencil-box = "0.1.6"
```

> Replace `"0.1.6"` with the latest version from [crates.io](https://crates.io/crates/pencil-box)

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

#### `chunk` Function Behavior

```rust
pub fn chunk<T: Clone>(array: &[T], chunk_size: usize) -> Result<Vec<Vec<T>>, &'static str>
```

-   ✅ Returns an error if `chunk_size == 0`
-   ✅ Returns an empty vector if the input slice is empty
-   ✅ Returns a single chunk if `chunk_size >= array.len()`
-   ✅ Returns multiple chunks of up to `chunk_size` elements otherwise

Each chunk is cloned into an owned `Vec<T>`.

---

### 🔁 Deduplicating Vectors

```rust
use pencil_box::{uniq, uniq_performant};

fn main() {
    let mut items = vec![1, 2, 2, 3, 1];
    uniq(&mut items); // or uniq_performant(&mut items);
    assert_eq!(items, vec![1, 2, 3]);
}
```

#### `uniq` Function Behavior

```rust
pub fn uniq<T: Eq + Hash + Clone>(values: &mut Vec<T>)
```

-   ✅ Uses the standard `HashSet` to remove duplicates in-place
-   ✅ Retains the first occurrence of each element
-   ✅ Preserves relative ordering
-   🔁 Best for general-purpose use

#### `uniq_performant` Function Behavior

```rust
pub fn uniq_performant<T: Eq + Hash + Clone>(values: &mut Vec<T>)
```

-   🚀 Uses `AHashSet` for faster hashing and performance
-   ✅ Retains the first occurrence of each element
-   ✅ Preserves relative ordering
-   ⚡ Ideal for high-throughput or performance-critical use cases

---

### ➖ Computing Vector Differences

```rust
use pencil_box::{difference, difference_performant};

fn main() {
    let a = vec![1, 2, 3, 4];
    let b = vec![2, 4];
    let c = vec![5];
    let result = difference(&a, &vec![&b, &c]);
    // or: let result = difference_performant(&a, &vec![&b, &c]);
    assert_eq!(result, vec![1, 3]);
}
```

#### `difference` Function Behavior

```rust
pub fn difference<T: Eq + Hash + Clone>(to_compare: &Vec<T>, others: &Vec<&Vec<T>>) -> Vec<T>
```

-   ✅ Returns a new `Vec<T>` excluding any elements found in `others`
-   ✅ Uses standard `HashSet` for equality and lookup
-   ✅ Clones values into the output (fully owned)
-   ✅ Best for secure, general-purpose usage
-   ⚡ Preallocates memory for performance

#### `difference_performant` Function Behavior

```rust
pub fn difference_performant<T: Eq + Hash + Clone>(to_compare: &Vec<T>, others: &Vec<&Vec<T>>) -> Vec<T>
```

-   🚀 Uses `AHashSet` for faster lookup performance
-   ✅ Identical output to `difference`
-   ✅ Clones values for full ownership
-   ⚠️ Not cryptographically secure (faster but non-resistant to hash DoS attacks)
-   ⚡ Best for large or performance-critical data sets

---

### 🗑️ Compacting Vectors

```rust
use pencil_box::compact;

fn main() {
    let mut numbers = vec![1, 0, 2, 0, 3];
    compact(&mut numbers);
    assert_eq!(numbers, vec![1, 2, 3]);

    let mut strings = vec!["hello".to_string(), "".to_string(), "world".to_string()];
    compact(&mut strings);
    assert_eq!(strings, vec!["hello".to_string(), "world".to_string()]);
}
```

#### `compact` Function Behavior

```rust
pub fn compact<T: IsEmpty>(values: &mut Vec<T>)
```

-   🗑️ Removes elements from a mutable vector that are considered "empty"
-   ✅ Modifies the vector in-place
-   ✅ Uses the `IsEmpty` trait for custom emptiness logic
-   ✅ Works with `String`, `&str`, `Vec<T>`, `Option<T>`, `bool`, integers, and floats
-   ⚡ Efficient: uses `Vec::retain()` under the hood

## 🧪 ✅ Running Tests

To run all unit tests 

```
cargo test --tests
```

---

## 🔒 Safety

-   ✅ 100% safe Rust (`#![forbid(unsafe_code)]`)
-   ✅ No `unsafe` blocks used
-   ✅ Pure functional logic

---

## 📄 License

-   [MIT](LICENSE-MIT)

---

## 🤝 Contributing

Contributions, bug reports, and feature requests are welcome.  
Please open an issue or submit a pull request.

---

## 🌐 Links

-   [Crates.io](https://crates.io/crates/pencil-box)
-   [Repository](https://github.com/rocketnozzle/pencil-box)