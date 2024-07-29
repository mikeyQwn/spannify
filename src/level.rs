//! Span levels

/// Repesent a level of the span. The level determines if the span should be ouputted or not. If
/// `Span` level is less than parent `Spanner` level, the span is ignored.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Level {
    Trace,
    Debug,
    Info,
    Warn,
    Error,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sanity_check() {
        assert!(Level::Trace < Level::Warn);
    }
}
