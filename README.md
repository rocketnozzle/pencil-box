
[![Rust](https://github.com/rocketnozzle/pencil-box/actions/workflows/rust.yml/badge.svg)](https://github.com/rocketnozzle/pencil-box/actions/workflows/rust.yml)

# ✏️ Pencil Box

A performance-focused, memory-efficient utility library for Rust — built for engineers who care about speed, safety, and maintainability.


---

> ✨🌟🌟🌟 If you find this project useful, please consider [⭐ starring it on GitHub](https://github.com/rocketnozzle/pencil-box)! 🌟🌟🌟✨

---

## 📚 Documentation

👉 [**Full API Reference**](https://docs.rs/pencil-box/latest/pencil_box/)

---

## 🚀 Why Choose This Library?

While there are many libraries out there, this one is built different — with engineers in mind:

### ✅ Performance First
- Designed for speed and minimal latency
- Zero-cost abstractions where possible

### 🧠 Memory-Efficient by Design
- Built to minimize allocations and overhead
- Smart data structures and lean processing

### 🛡️ Safety Comes Standard
- No `unsafe` blocks unless absolutely necessary — and always documented

### ⏱️ 48-Hour Response Promise
- 🐛 Bug reports: Responded to **within 48 hours**
- 💡 Feature requests: Reviewed and responded to in **under 2 days**
- 📦 Developer-first and actively maintained

> ⚡ **Performance-focused.** 💾 **Memory-conscious.** 🛡️ **Safe by design.** 🔁 **Backed by a 48-hour support promise.**


---
## 📦 Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
pencil-box = "0.1.11"
```

> Replace `"0.1.11"` with the latest version from [crates.io](https://crates.io/crates/pencil-box).


Full feature list is available in the crate’s `Cargo.toml`.

---

## 🚀 Usage & Available Methods

Each function is currently part of the `array` module. Full documentation with examples is available via the official [docs.rs documentation](https://docs.rs/pencil-box).

| Component | Function                | Description                                                 | Full API docs / API Reference                                                                                |
| --------- | ----------------------- | ----------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------ |
| array     | `chunk`                 | Split slices into fixed-size chunks                         | [Full API Docs](https://docs.rs/pencil-box/latest/pencil_box/array/chunk/fn.chunk.html)                      |
| array     | `compact`               | Remove "empty" values using the `IsEmpty` trait             | [Full API Docs](https://docs.rs/pencil-box/latest/pencil_box/array/compact/fn.compact.html)                  |
| array     | `difference_performant` | Faster list difference using `AHashSet`                     | [Full API Docs](https://docs.rs/pencil-box/latest/pencil_box/array/difference/fn.difference_performant.html) |
| array     | `difference`            | Compute list difference using `HashSet`                     | [Full API Docs](https://docs.rs/pencil-box/latest/pencil_box/array/difference/fn.difference.html)            |
| array     | `drop_end`              | Remove N elements from the end of a vector                  | [Full API Docs](https://docs.rs/pencil-box/latest/pencil_box/array/drop_end/fn.drop_end.html)                |
| array     | `drop_start`            | Remove N elements from the beginning of a vector            | [Full API Docs](https://docs.rs/pencil-box/latest/pencil_box/array/drop_start/fn.drop_start.html)            |
| array     | `fill_default`          | Fill a vector with `T::default()` values                    | [Full API Docs](https://docs.rs/pencil-box/latest/pencil_box/array/fill/fn.fill_default.html)                |
| array     | `fill_value`            | Fill a vector with clones of a given value                  | [Full API Docs](https://docs.rs/pencil-box/latest/pencil_box/array/fill/fn.fill_value.html)                  |
| array     | `find_index`            | Find the index of the **first** matching element            | [Full API Docs](https://docs.rs/pencil-box/latest/pencil_box/array/find/fn.find_index.html)                  |
| array     | `find_indexes`          | Find indices of **all** matching elements                   | [Full API Docs](https://docs.rs/pencil-box/latest/pencil_box/array/find/fn.find_indexes.html)                |
| array     | `find_last_index`       | Find the index of the **last** matching element             | [Full API Docs](https://docs.rs/pencil-box/latest/pencil_box/array/find/fn.find_last_index.html)             |
| array     | `flatten`               | Flatten nested array-like structures into a single `Vec<T>` | [Full API Docs](https://docs.rs/pencil-box/latest/pencil_box/array/flatten/fn.flatten.html)                  |
| array     | `intersection`          | Compute elements common to **all** input collections        | [Full API Docs](https://docs.rs/pencil-box/latest/pencil_box/array/intersection/fn.intersection.html)        |
| array     | `uniq_performant`       | Faster deduplication using `AHashSet`                       | [Full API Docs](https://docs.rs/pencil-box/latest/pencil_box/array/uniq/fn.uniq_performant.html)             |
| array     | `uniq`                  | Remove duplicate elements using `HashSet`                   | [Full API Docs](https://docs.rs/pencil-box/latest/pencil_box/array/uniq/fn.uniq.html)                        |

---

## 🧪 Running Tests

```bash
cargo test --tests
```

---

## 📊 Code Coverage

To generate a coverage report:

```bash
cargo tarpaulin --out html --output-dir code_coverage
```

---

## 🔒 Safety

- ✅ 100% safe Rust (`#![forbid(unsafe_code)]`)
- ✅ No `unsafe` blocks
- ✅ Panic-free under all valid inputs

---

## 🤝 Contributing

I welcome contributions, bug reports, and feature requests!

- Open an issue
- Submit a PR
- Or just say hi — I typically respond in **under 48 hours**

---

## 🌐 Links

- 📦 [Crates.io](https://crates.io/crates/pencil-box)
- 📁 [GitHub Repository](https://github.com/rocketnozzle/pencil-box)
- 📘 [Documentation](https://docs.rs/pencil-box/latest/pencil_box/)
