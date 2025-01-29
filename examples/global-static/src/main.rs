use spannify::{config::Config, core::StdoutSpanner, spf};
use std::sync::LazyLock;

static SPANNER: LazyLock<StdoutSpanner> =
    LazyLock::new(|| StdoutSpanner::new().with_config(Config::new().with_skip(1)));

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
