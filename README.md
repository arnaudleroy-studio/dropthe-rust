# dropthe

Rust client for [DropThe](https://dropthe.org) open data API.

Access movies, series, crypto, and company data through a simple Rust interface.

## Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
dropthe = "0.1.0"
```

## Usage

```rust
use dropthe::{VERSION, BASE_URL};

fn main() {
    println!("DropThe client v{}", VERSION);
    println!("API: {}", BASE_URL);
}
```

## Links

- **Homepage**: [https://dropthe.org](https://dropthe.org)
- **Data API**: [https://dropthe.org/data/](https://dropthe.org/data/)
- **Repository**: [https://github.com/arnaudleroy-studio/dropthe-rust](https://github.com/arnaudleroy-studio/dropthe-rust)

## License

MIT - See [LICENSE](LICENSE) for details.
