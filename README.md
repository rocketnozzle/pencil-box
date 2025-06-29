# Pencil-Box

[![Rust](https://github.com/rocketnozzle/pencil-box/actions/workflows/rust.yml/badge.svg)](https://github.com/rocketnozzle/pencil-box/actions/workflows/rust.yml)

A high-performance, idiomatic Rust utility crate offering Lodash-style collection and value manipulation functions.

## 📚 Documentation

👉 [Full API docs / Reference](https://docs.rs/pencil-box/latest/pencil_box/)

---

## 📦 Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
pencil-box = "0.1.6"
```

> Replace `"0.1.6"` with the latest version from [crates.io](https://crates.io/crates/pencil-box).

---

## 🚀 Usage & Available Methods

Each function is currently part of the `array` module. Full documentation with examples is available via the official [docs.rs documentation](https://docs.rs/pencil-box).

| Component | Function                | Description                                      | Full API docs / API Reference                                                                                         |
| --------- | ----------------------- | ------------------------------------------------ | --------------------------------------------------------------------------------------------------------------------- |
| array     | `chunk`                 | Split slices into fixed-size chunks              | [Full API Docs](https://docs.rs/pencil-box/latest/pencil_box/array/chunk/fn.chunk.html)                                       |
| array     | `uniq`                  | Remove duplicate elements using `HashSet`        | [Full API Docs](https://docs.rs/pencil-box/latest/pencil_box/array/uniq/fn.uniq.html)                                          |
| array     | `uniq_performant`       | Faster deduplication using `AHashSet`            | [Full API Docs](https://docs.rs/pencil-box/latest/pencil_box/array/uniq/fn.uniq_performant.html)                   |
| array     | `difference`            | Compute list difference using `HashSet`          | [Full API Docs](https://docs.rs/pencil-box/latest/pencil_box/array/difference/fn.difference.html)                        |
| array     | `difference_performant` | Faster list difference using `AHashSet`          | [Full API Docs](https://docs.rs/pencil-box/latest/pencil_box/array/difference/fn.difference_performant.html) |
| array     | `compact`               | Remove "empty" values using the `IsEmpty` trait  | [Full API Docs](https://docs.rs/pencil-box/latest/pencil_box/array/compact/fn.compact.html)                                 |
| array     | `drop_start`            | Remove N elements from the beginning of a vector | [Full API Docs](https://docs.rs/pencil-box/latest/pencil_box/array/drop_start/fn.drop_start.html)                       |
| array     | `drop_end`              | Remove N elements from the end of a vector       | [Full API Docs](https://docs.rs/pencil-box/latest/pencil_box/array/drop_end/fn.drop_end.html)                             |

---


## 🥪✅ Running Tests

To run all unit tests, use:

```bash
cargo test --tests
```

---

## 💯📂 Code Coverage

To generate a code coverage report, run:

```bash
cargo tarpaulin --out html --output-dir code_coverage
```

---

## 🔒 Safety

* ✅ 100% safe Rust (`#![forbid(unsafe_code)]`)
* ✅ No `unsafe` blocks
* ✅ Panic-free under all valid inputs

---

## 📄 License

Licensed under [MIT](LICENSE-MIT).

---

## 🤝 Contributing

Contributions, bug reports, and feature requests are welcome!
Open an issue or submit a pull request anytime.

---

## 🌐 Links

* [Crates.io](https://crates.io/crates/pencil-box)
* [Repository](https://github.com/rocketnozzle/pencil-box)
* [Documentation](https://docs.rs/pencil-box/latest/pencil_box/)
