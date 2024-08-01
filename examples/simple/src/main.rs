use spannify::core::StdoutSpanner;

fn fib(s: &StdoutSpanner, x: usize) -> usize {
    let _span = s.enter_span(format!("fib({x})").as_ref());
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
