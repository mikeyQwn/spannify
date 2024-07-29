# Spannify

![Build status][actions-badge]

[actions-badge]: https://github.com/mikeyQwn/spannify/actions/workflows/ci.yml/badge.svg

A tiny rust crate that produces nice-looking graphs for you to visualize your callstack

### Example

```rust
 use spannify::core::StdoutSpanner;

 fn fib(s: &StdoutSpanner, x: usize) -> usize {
     let _span = s.enter_span(format!("fib({})", x).as_ref());
     match x {
         0 => 0,
         1 | 2 => 1,
         _ => fib(s, x - 1) + fib(s, x - 2),
     }
 }


 fn main() {
     let spanner = StdoutSpanner::new();

     let _ = fib(&spanner, 5);
 }

```

### Output

```text
┌fib(5)
|  fib(4)
|   ┌fib(3)
|   ┆  fib(2)
|   ┆  fib(2)
|   ┆  fib(1)
|   ┆  fib(1)
|   └fib(3)
|   ┌fib(2)
|   └fib(2)
|  fib(4)
|  fib(3)
|   ┌fib(2)
|   └fib(2)
|   ┌fib(1)
|   └fib(1)
|  fib(3)
└fib(5)
```

### Documentation

Check out the full documentation at [docs.rs](https://docs.rs/spannify/latest/spannify/)
