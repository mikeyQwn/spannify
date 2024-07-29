use core::cell::RefCell;
use std::sync::atomic::{AtomicUsize, Ordering};

use crate::config::Config;

pub struct Spanner<T>
where
    T: std::io::Write,
{
    writer: RefCell<T>,
    depth: AtomicUsize,
    config: Config,
}

impl<T> Spanner<T>
where
    T: std::io::Write,
{
    pub fn from_writer(writer: T) -> Self {
        Self {
            writer: RefCell::new(writer),
            depth: AtomicUsize::new(0),
            config: Config::default(),
        }
    }

    pub fn enter_span(&self, name: &str) -> Span<T> {
        Span::enter(&self, name)
    }

    pub fn with_config(self, cfg: Config) -> Self {
        Self {
            writer: self.writer,
            depth: self.depth,
            config: cfg,
        }
    }
}

pub type VecSpanner = Spanner<Vec<u8>>;

impl VecSpanner {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn from_vec(vec: Vec<u8>) -> Self {
        Self {
            writer: RefCell::new(vec),
            depth: AtomicUsize::new(0),
            config: Config::default(),
        }
    }
}

impl Default for VecSpanner {
    fn default() -> Self {
        Self {
            writer: RefCell::new(Vec::new()),
            depth: AtomicUsize::new(0),
            config: Config::default(),
        }
    }
}

pub type FileSpanner = Spanner<std::fs::File>;

impl FileSpanner {
    pub fn new(file: std::fs::File) -> Self {
        Self {
            writer: RefCell::new(file),
            depth: AtomicUsize::new(0),
            config: Config::default(),
        }
    }
}

pub type StdoutSpanner = Spanner<std::io::Stdout>;

impl StdoutSpanner {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Default for StdoutSpanner {
    fn default() -> Self {
        Self {
            writer: RefCell::new(std::io::stdout()),
            depth: AtomicUsize::new(0),
            config: Config::default(),
        }
    }
}

pub struct Span<'a, T>
where
    T: std::io::Write,
{
    parent: &'a Spanner<T>,
    drop_message: String,
}

impl<'a, T> Span<'a, T>
where
    T: std::io::Write,
{
    pub fn enter(parent: &'a Spanner<T>, name: &str) -> Self {
        let prev_depth = parent.depth.fetch_add(1, Ordering::Relaxed);
        let (enter_message, drop_message) =
            Self::generate_messages(name, prev_depth, &parent.config);

        let _ = parent.writer.borrow_mut().write(enter_message.as_ref());
        Self {
            parent,
            drop_message,
        }
    }

    fn generate_messages(name: &str, depth: usize, cfg: &Config) -> (String, String) {
        let spaces: String =
            (0..depth).fold(String::with_capacity(depth * cfg.tabwidth), |mut acc, _| {
                for _ in 0..cfg.tabwidth {
                    acc.push(' ')
                }
                acc
            });
        let enter_message = format!("{}{} entered\n", spaces, name);
        let drop_message = format!("{}{} dropped\n", spaces, name);
        (enter_message, drop_message)
    }
}

impl<'a, T> Drop for Span<'a, T>
where
    T: std::io::Write,
{
    fn drop(&mut self) {
        let _ = self.parent.depth.fetch_sub(1, Ordering::Relaxed);
        let _ = self
            .parent
            .writer
            .borrow_mut()
            .write(self.drop_message.as_ref());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct Helper {
        spanner: VecSpanner,
    }
    impl Helper {
        fn helper(&self, current_depth: usize, target_depth: usize) {
            let _span = self
                .spanner
                .enter_span(format!("Span({})", current_depth).as_ref());

            if current_depth >= target_depth {
                return;
            }
            return self.helper(current_depth + 1, target_depth);
        }
    }

    #[test]
    fn it_works() {
        let helper = Helper {
            spanner: VecSpanner::new(),
        };

        let expected = r#"Span(0) entered
  Span(1) entered
    Span(2) entered
      Span(3) entered
        Span(4) entered
          Span(5) entered
          Span(5) dropped
        Span(4) dropped
      Span(3) dropped
    Span(2) dropped
  Span(1) dropped
Span(0) dropped
"#;

        helper.helper(0, 5);
        assert_eq!(
            expected.bytes().collect::<Vec<_>>(),
            helper.spanner.writer.into_inner()
        )
    }
}
