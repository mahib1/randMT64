# randMT64

A lightweight, fast, and high-resolution pseudo-random number generator in Rust based on the **Mersenne Twister 64-bit** algorithm — with support for `u128` outputs and floating-point results mapped to any configurable range.

**Crate name:** `randMT64`  
**Module root:** `rand_num_gen`  
**Author:** [@mahib1](https://github.com/mahib1)

---

## 🌟 Features

- ⚙️ Pure Rust implementation of MT19937-64
- 🔐 Generates 128-bit numbers by combining two 64-bit outputs
- 🕒 Microsecond-precision time-based seeding using `SystemTime`
- 🎚 Range mapping to produce `f64` outputs in any interval
- 🧪 Deterministic, seedable output — perfect for simulations and research
- 🪶 Lightweight and dependency-free (except `std`)

---

## 📦 Usage

Add to your `Cargo.toml`:

```toml
[dependencies]
randMT64 = "0.1.0"
````

Use in code:

```rust
use randMT64::{rand, RandomRange};

fn main() {
    let r1 = rand(RandomRange::default()).unwrap();
    println!("Random float [0, 1): {}", r1);

    let r2 = rand(RandomRange::new(10, 100)).unwrap();
    println!("Random float [10, 100): {}", r2);
}
```

---

## 📁 Crate Structure

```
randMT64/
├── src/
│   ├── lib.rs           # Public API
│   ├── mt64.rs          # Mersenne Twister 64-bit implementation
│   ├── time.rs          # SystemTime-based seeding
│   └── range.rs         # RandomRange config and mapping
├── examples/
│   └── usage.rs         # Sample usage
├── Cargo.toml
└── README.md
```

---

## 🔧 API Overview

### `rand(config: RandomRange) -> Result<f64, std::io::Error>`

* Generates a pseudo-random `f64` in the interval \[`config.start`, `config.end`)
* Internally uses the current system time (in microseconds) to seed a new MT64 instance

### `RandomRange::new(start: u128, end: u128)`

* Configure a custom output range

### `RandomRange::default()`

* Shortcut for range `[0, 1)` using the `MAGIC` resolution constant

---

## 📜 License

Licensed under either of:

* MIT License ([LICENSE-MIT](LICENSE-MIT))
* Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE))

---

## 🚧 TODOs

* [ ] Optional persistent seeding
* [ ] Support for `no_std`
* [ ] Unit and randomness quality tests
* [ ] Expose `MersenneTwister64` directly via the crate root

---

## 🤔 Why `randMT64`?

> Because `rand` is too big, and `rand_core` is too low-level.
> This is the minimalist, high-precision, configurable `u128` solution you didn’t know you needed.

---

## 👤 Author

Built by [@mahib1](https://github.com/mahib1)



