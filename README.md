# Spannify

[![Build status][actions-badge]][actions]
[![Crates.io][version-badge]][crates.io]
![License][license-badge]

[actions-badge]: https://github.com/mikeyQwn/spannify/actions/workflows/ci.yml/badge.svg
[actions]: https://github.com/mikeyQwn/spannify/actions?query=branch%3Amaster
[version-badge]: https://img.shields.io/crates/v/spannify.svg
[crates.io]: https://crates.io/crates/spannify
[license-badge]: https://img.shields.io/github/license/mikeyQwn/spannify.svg

A tiny rust crate that produces nice-looking graphs for you to visualize your callstack

### Example

```rust
use once_cell::sync::Lazy;
use spannify::{config::Config, core::StdoutSpanner, spf};

static SPANNER: Lazy<StdoutSpanner> =
    Lazy::new(|| StdoutSpanner::new().with_config(Config::new().with_skip(1)));

fn fib(n: usize) -> usize {
    let _span = spf!(SPANNER, "fib({n})");
    match n {
        0 => 0,
        1 | 2 => 1,
        _ => fib(n - 1) + fib(n - 2),
    }
}

fn main() {
    fib(5);
}
```

### Output

```text
┌fib(5)
| ┌fib(4)
| ¦ ┌fib(3)
| ¦ ┆ ┌fib(2)
| ¦ ┆ └fib(2)
| ¦ ┆ ┌fib(1)
| ¦ ┆ └fib(1)
| ¦ └fib(3)
| ¦ ┌fib(2)
| ¦ └fib(2)
| └fib(4)
| ┌fib(3)
| ¦ ┌fib(2)
| ¦ └fib(2)
| ¦ ┌fib(1)
| ¦ └fib(1)
| └fib(3)
└fib(5)
```

### Documentation

Check out the full documentation at [docs.rs](https://docs.rs/spannify/latest/spannify/)
Or take a look at the examples in `examples`
