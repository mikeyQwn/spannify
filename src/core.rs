//! Span and span generators
//!
//! This module provides functionality for generating spans and keeping track of the span depth.
//! It includes the `Spanner` struct for managing span creation and the `Span` struct for representing individual spans.

use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Mutex;

use crate::config::Config;

/// A structure that generates spans and keeps track of the span depth.
#[derive(Debug)]
pub struct Spanner<T>
where
    T: std::io::Write,
{
    writer: Mutex<T>,
    depth: AtomicUsize,
    config: Config,
}

impl<T> Spanner<T>
where
    T: std::io::Write,
{
    /// Creates a new `Spanner` instance from a writer.
    ///
    /// # Parameters
    /// - `writer`: The writer to which the spans will be written.
    ///
    /// # Examples
    /// ```
    /// use spannify::core::Spanner;
    ///
    /// let spanner = Spanner::from_writer(Vec::new());
    /// ```
    pub fn from_writer(writer: T) -> Self {
        Self {
            writer: Mutex::new(writer),
            depth: AtomicUsize::new(0),
            config: Config::default(),
        }
    }

    /// Enters a span, increasing the depth and writing the span's enter message.
    ///
    /// # Parameters
    /// - `name`: The name of the span. It is displayed is span's enter and exit message
    ///
    /// # Examples
    /// ```
    /// use spannify::core::Spanner;
    ///
    /// let spanner = Spanner::from_writer(Vec::new());
    /// let span = spanner.enter_span("test");
    /// ```
    pub fn enter_span(&self, name: &str) -> Span<T> {
        Span::enter(self, name)
    }

    /// Sets a custom configuration for the spanner.
    ///
    /// # Parameters
    /// - `cfg`: The new configuration to use.
    ///
    /// # Examples
    /// ```
    /// use spannify::{config::Config, core::Spanner};
    ///
    /// let spanner = Spanner::from_writer(Vec::new()).with_config(Config::new().with_skip(3));
    /// ```
    #[must_use]
    pub fn with_config(self, cfg: Config) -> Self {
        Self {
            writer: self.writer,
            depth: self.depth,
            config: cfg,
        }
    }
}

/// A Spanner that writes to the Vec of bytes.
pub type VecSpanner = Spanner<Vec<u8>>;

impl VecSpanner {
    /// Creates a `VecSpanner` instance with default values.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Creates a new `VecSpanner` from an existing vector.
    ///
    /// # Parameters
    /// - `vec`: The vector to use as the writer.
    ///
    /// # Examples
    /// ```
    /// use spannify::core::VecSpanner;
    ///
    /// let spanner = VecSpanner::from_vec(Vec::new());
    /// ```
    #[must_use]
    pub fn from_vec(vec: Vec<u8>) -> Self {
        Self {
            writer: Mutex::new(vec),
            depth: AtomicUsize::new(0),
            config: Config::default(),
        }
    }
}

impl Default for VecSpanner {
    fn default() -> Self {
        Self {
            writer: Mutex::new(Vec::new()),
            depth: AtomicUsize::new(0),
            config: Config::default(),
        }
    }
}

/// A Spanner that writes to a File
pub type FileSpanner = Spanner<std::fs::File>;

impl FileSpanner {
    /// Creates a new `FileSpanner` from an existing file.
    ///
    /// # Parameters
    /// - `file`: The file to use as the writer.
    ///
    /// # Examples
    /// ```
    /// use spannify::core::FileSpanner;
    ///
    /// let file = std::fs::File::create("/tmp/output.txt").unwrap();
    /// let spanner = FileSpanner::new(file);
    /// ```
    #[must_use]
    pub fn new(file: std::fs::File) -> Self {
        Self {
            writer: Mutex::new(file),
            depth: AtomicUsize::new(0),
            config: Config::default(),
        }
    }
}

/// A Spanner that writes to the standard out.
pub type StdoutSpanner = Spanner<std::io::Stdout>;

impl StdoutSpanner {
    /// Creates a `StdoutSpanner` instance with default values.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }
}

impl Default for StdoutSpanner {
    fn default() -> Self {
        Self {
            writer: Mutex::new(std::io::stdout()),
            depth: AtomicUsize::new(0),
            config: Config::default(),
        }
    }
}

/// A `Span` represents a hierarchical structure for tracking and displaying the entry and
/// exit of various sections of code. It uses a provided writer to output messages on entering
/// and dropping spans, facilitating visual traceability of the execution flow.
///
/// The `Span` structure works in tandem with a `Spanner` instance, which maintains the
/// configuration and depth state. Each span generates formatted messages based on the
/// current depth and configuration, which are written to the provided writer.
#[derive(Clone, Debug)]
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
    /// Creates a new `Span` for a given `parent` `Spanner` and a span `name`. This method
    /// increases the depth of the parent spanner, generates entry and drop messages, writes
    /// the entry message to the writer, and returns the new `Span` instance.
    ///
    /// # Parameters
    ///
    /// - `parent`: A reference to the `Spanner` instance managing the configuration and depth.
    /// - `name`: The name of the span, which will be included in the messages.
    fn enter(parent: &'a Spanner<T>, name: &str) -> Self {
        let prev_depth = parent.depth.fetch_add(1, Ordering::Relaxed);
        let (enter_message, drop_message) =
            Self::generate_messages(name, prev_depth, &parent.config);

        if let Ok(mut writer) = parent.writer.lock() {
            let _ = writer.write(enter_message.as_ref());
        }
        Self {
            parent,
            drop_message,
        }
    }
    /// Generates the entry and drop messages for a span based on its name, depth, and configuration.
    ///
    /// # Parameters
    ///
    /// - `name`: The name of the span.
    /// - `depth`: The current depth of the span.
    /// - `cfg`: The configuration for formatting the messages.
    ///
    fn generate_messages(name: &str, depth: usize, cfg: &Config) -> (String, String) {
        let spaces: String = (0..depth).enumerate().fold(
            String::with_capacity(depth * cfg.tabwidth),
            |mut acc, (i, _)| {
                let is_displayed = i % cfg.skip == 0;
                if is_displayed {
                    acc.push((cfg.depthmap)(i));
                } else {
                    acc.push(' ');
                }
                for _ in 0..cfg.tabwidth.saturating_sub(1) {
                    acc.push(' ');
                }
                acc
            },
        );
        let is_displayed = depth % cfg.skip == 0;
        let enter_message = format!(
            "{}{}{} entered\n",
            spaces,
            if is_displayed { '┌' } else { ' ' },
            name
        );
        let drop_message = format!(
            "{}{}{} dropped\n",
            spaces,
            if is_displayed { '└' } else { ' ' },
            name
        );
        (enter_message, drop_message)
    }
}

/// Implements the `Drop` trait for the `Span` struct, ensuring that the drop message is
/// written to the writer and the parent's depth is decremented when the span goes out of scope.
impl<'a, T> Drop for Span<'a, T>
where
    T: std::io::Write,
{
    /// Writes the drop message to the writer and decrements the parent's depth.
    fn drop(&mut self) {
        let _ = self.parent.depth.fetch_sub(1, Ordering::Relaxed);

        if let Ok(mut writer) = self.parent.writer.lock() {
            let _ = writer.write(self.drop_message.as_ref());
        }
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

        let expected = r#"┌Span(0) entered
|  Span(1) entered
|   ┌Span(2) entered
|   ┆  Span(3) entered
|   ┆   ┌Span(4) entered
|   ┆   |  Span(5) entered
|   ┆   |  Span(5) dropped
|   ┆   └Span(4) dropped
|   ┆  Span(3) dropped
|   └Span(2) dropped
|  Span(1) dropped
└Span(0) dropped
"#;

        helper.helper(0, 5);
        let vec = helper.spanner.writer.into_inner().unwrap();
        assert_eq!(expected.bytes().collect::<Vec<_>>(), vec)
    }

    #[test]
    fn test_config() {
        let helper = Helper {
            spanner: VecSpanner::new().with_config(Config::new().with_skip(3)),
        };

        let expected = r#"┌Span(0) entered
|  Span(1) entered
|    Span(2) entered
|     ┌Span(3) entered
|     ┊  Span(4) entered
|     ┊    Span(5) entered
|     ┊    Span(5) dropped
|     ┊  Span(4) dropped
|     └Span(3) dropped
|    Span(2) dropped
|  Span(1) dropped
└Span(0) dropped
"#;

        helper.helper(0, 5);
        let vec = helper.spanner.writer.into_inner().unwrap();
        assert_eq!(expected.bytes().collect::<Vec<_>>(), vec)
    }
}
