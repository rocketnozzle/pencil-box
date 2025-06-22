# Pencil-Box

A simple and efficient Rust utility for:

-   📦 Splitting slices into fixed-size chunks with `chunk`

-   🔁 Removing duplicate elements from vectors with `uniq` and `uniq_performant`

-   🗑️ Removing "empty" elements from vectors with `compact`


## 📦 Installation

Add this to your `Cargo.toml`:

```
[dependencies]
pencil-box = "0.1.4"


```

> Replace `"0.1.4"` with the latest version from [crates.io](https://crates.io/crates/pencil-box "null")

## 🚀 Usage

### ✂️ Chunking Slices

```
use pencil_box::chunk;

fn main() {
    let data = vec![1, 2, 3, 4, 5];
    let result = chunk(&data, 2).unwrap();
    assert_eq!(result, vec![vec![1, 2], vec![3, 4], vec![5]]);
}


```

#### `chunk` Function Behavior

```
pub fn chunk<T: Clone>(array: &[T], chunk_size: usize) -> Result<Vec<Vec<T>>, &'static str>


```

-   ✅ Returns an error if `chunk_size == 0`

-   ✅ Returns an empty vector if the input slice is empty

-   ✅ Returns a single chunk if `chunk_size >= array.len()`

-   ✅ Returns multiple chunks of up to `chunk_size` elements otherwise


Each chunk is cloned into an owned `Vec<T>`.

### 🧹 Deduplicating Vectors

```
use pencil_box::{uniq, uniq_performant};

fn main() {
    let mut items = vec![1, 2, 2, 3, 1];
    uniq(&mut items); // or uniq_performant(&mut items);
    assert_eq!(items, vec![1, 2, 3]);
}


```

#### `uniq` Function Behavior

```
pub fn uniq<T: Eq + Hash + Clone>(values: &mut Vec<T>)


```

-   ✅ Uses the standard `HashSet` to remove duplicates in-place

-   ✅ Retains the first occurrence of each element

-   ✅ Preserves relative ordering

-   🔁 Best for general-purpose use


#### `uniq_performant` Function Behavior

```
pub fn uniq_performant<T: Eq + Hash + Clone>(values: &mut Vec<T>)


```

-   🚀 Uses `AHashSet` for faster hashing and performance

-   ✅ Retains the first occurrence of each element

-   ✅ Preserves relative ordering

-   ⚡ Ideal for high-throughput or performance-critical use cases


### 🗑️ Compacting Vectors

```
use pencil_box::compact; // Assuming compact is part of pencil_box

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

```
pub fn compact<T: IsEmpty>(values: &mut Vec<T>)


```

-   🗑️ Removes elements from a mutable vector that are considered "empty"

-   ✅ Modifies the vector in-place

-   ✅ Relies on the `IsEmpty` trait to determine emptiness

-   Supported types for `IsEmpty` include `String`, `&str`, `Vec<T>`, `bool`, all integer types (`i8`, `u8`, etc.), all float types (`f32`, `f64`), and `Option<T>`.

-   ⚡ Efficient: Uses `Vec::retain()` for in-place removal.


## 🔒 Safety

-   100% safe Rust (`#![forbid(unsafe_code)]`)

-   No `unsafe` blocks used

-   Pure functional logic


## 📄 License
-   [MIT](LICENSE-MIT "null")

## 🤝 Contributing

Contributions, bug reports, and feature requests are welcome. Please open an issue or submit a pull request.

## 🌐 Links

-   [Crates.io](https://crates.io/crates/pencil-box "null")

-   [Repository](https://github.com/rocketnozzle/pencil-box "null")