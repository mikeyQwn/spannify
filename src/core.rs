//! Span and span generators
//!
//! This module provides functionality for generating spans and keeping track of the span depth.
//! It includes the `Spanner` struct for managing span creation and the `Span` struct for representing individual spans.

use std::fmt::Arguments;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Mutex;

use crate::config::Config;
use crate::level::Level;

/// A structure that generates spans and keeps track of the span depth.
///
/// Note that `Spanner` is thread-safe, not because it produces coherent traces,
/// but to make using it as a global mutable variable less cumbersome.
/// If you want to use `Spanner` in multithreaded environments, you should consider
/// creating a separate instance for each thread

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

    /// Enters a span with `Level::Info`, increasing the depth and writing the span's enter message.
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
        self.enter_with_level(Level::Info, name)
    }

    /// Enters a span with `level`, increasing the depth and writing the span's enter message if
    /// level is not less than `Spanner.level`.
    ///
    /// # Parameters
    /// - `level`: The level of the span. It determines if the span would be outputted or not
    /// - `name`: The name of the span. It is displayed is span's enter and exit message
    ///
    /// # Examples
    /// ```
    /// use spannify::{
    ///     level::Level,
    ///     config::Config,
    ///     core::Spanner,
    /// };
    /// use std::io::Cursor;
    ///
    /// let mut writer = Cursor::new(Vec::new());
    /// let spanner = Spanner::from_writer(&mut writer)
    ///     .with_config(Config::new().with_skip(3).with_level(Level::Info));
    ///
    /// {
    ///     let _span = spanner.enter_with_level(Level::Debug, "foo");
    ///     // Span is dropped here
    /// }
    ///
    /// assert_eq!(writer.get_ref(), &[]);
    /// ```
    pub fn enter_with_level(&self, level: Level, name: &str) -> Span<T> {
        Span::enter(self, level, name)
    }

    /// Does the same thing as `enter_with_level`, but uses `std::fmt::Arguments` instead of `&str`
    /// as a name to avoid allocations where possible in spf! macro.
    ///
    /// It is not
    pub fn enter_args(&self, level: Level, args: Arguments) -> Span<T> {
        Span::enter_args(self, level, args)
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
            config: cfg,
            ..self
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
/// exit of various sections of code.
///
/// It uses a provided writer to output messages on entering
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
    level: Level,
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
    /// - `level`: The level of the span.
    /// - `name`: The name of the span, which will be included in the messages.
    fn enter(parent: &'a Spanner<T>, level: Level, name: &str) -> Self {
        Self::enter_args(parent, level, format_args!("{name}"))
    }

    fn enter_args(parent: &'a Spanner<T>, level: Level, args: Arguments) -> Self {
        let mut drop_message = String::new();
        if parent.config.level <= level {
            let prev_depth = parent.depth.fetch_add(1, Ordering::Relaxed);
            let (enter_message, drop_msg) =
                Self::generate_messages(args, prev_depth, &parent.config);
            drop_message = drop_msg;

            if let Ok(mut writer) = parent.writer.lock() {
                let _ = writer.write(enter_message.as_ref());
            }
        }
        Self {
            parent,
            drop_message,
            level,
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
    fn generate_messages(name: Arguments, depth: usize, cfg: &Config) -> (String, String) {
        let spaces: String = (0..depth).enumerate().fold(
            String::with_capacity(depth * cfg.tabwidth),
            |mut acc, (i, _)| {
                let is_displayed = match cfg.skip {
                    0 => false,
                    _ => i % cfg.skip == 0,
                };
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
        let is_displayed = match cfg.skip {
            0 => false,
            _ => depth % cfg.skip == 0,
        };

        let enter_message = format!(
            "{}{}{}\n",
            spaces,
            if is_displayed { '┌' } else { ' ' },
            name
        );
        let drop_message = format!(
            "{}{}{}\n",
            spaces,
            if is_displayed { '└' } else { ' ' },
            name
        );
        (enter_message, drop_message)
    }
}

/// Implements the `Drop` trait for the `Span` struct, ensuring that the drop message is
/// written to the writer and the parent's depth is decremented when the span goes out of scope.
impl<T> Drop for Span<'_, T>
where
    T: std::io::Write,
{
    /// Writes the drop message to the writer and decrements the parent's depth.
    fn drop(&mut self) {
        if self.parent.config.level < self.level {
            return;
        }
        let _ = self.parent.depth.fetch_sub(1, Ordering::Relaxed);

        if let Ok(mut writer) = self.parent.writer.lock() {
            let _ = writer.write(self.drop_message.as_ref());
        }
    }
}

/// Creates a new span with a given spanner.
///
/// This macro is used to create and enter a new span.
/// You can specify the level of span if needed.
///
/// # Examples
///
/// ```rust
/// use spannify::{spf, level::Level, core::StdoutSpanner};
///
/// let my_spanner = StdoutSpanner::new();
///
/// let timestamp = "12:21";
///
/// // Creates a span with a level and literal message
/// spf!(my_spanner, Level::Info => "Starting process");
///
/// // Creates a span with a literal message
/// spf!(my_spanner => "Starting process");
///
/// // Example of creating a span with a formatted message
/// spf!(my_spanner, "Starting process at {}", timestamp);
/// // Or
/// spf!(my_spanner, "Starting process at {timestamp}");
///
/// // Example of creating a span with a level and formatted message
/// spf!(my_spanner, Level::Info, "Starting process at {}", timestamp);
///
#[macro_export]
macro_rules! spf {
    ($spa:expr, $level:path => $arg:literal) => {{
        let span = $spa.enter_with_level($level, $arg);
        span
    }};

    ($spa:expr => $arg:literal) => {{
        let span = $spa.enter_span($arg);
        span
    }};

    ($spa:expr, $level:path, $($arg:tt)*) => {{
        let span = $spa.enter_args($level, format_args!($($arg)*));
        span
    }};

    ($spa:expr, $($arg:tt)*) => {{
        let span = $spa.enter_args($crate::level::Level::Info, format_args!($($arg)*));
        span
    }};
}

#[cfg(test)]
mod tests {
    use std::io::Cursor;

    use super::*;

    struct Helper<T>
    where
        T: std::io::Write,
    {
        spanner: Spanner<T>,
    }
    impl<T> Helper<T>
    where
        T: std::io::Write,
    {
        fn helper(&self, current_depth: usize, target_depth: usize) {
            let _span = self
                .spanner
                .enter_span(format!("Span({})", current_depth).as_ref());

            if current_depth >= target_depth {
                return;
            }
            return self.helper(current_depth + 1, target_depth);
        }

        fn helper_macro(&self, current_depth: usize, target_depth: usize) {
            let _span = spf!(self.spanner, "Span({current_depth})");

            if current_depth >= target_depth {
                return;
            }
            return self.helper_macro(current_depth + 1, target_depth);
        }

        fn helper_macro_2(&self, current_depth: usize, target_depth: usize) {
            let _span = spf!(self.spanner, Level::Info, "Span({current_depth})");

            if current_depth >= target_depth {
                return;
            }
            return self.helper_macro_2(current_depth + 1, target_depth);
        }

        fn helper_macro_3(&self, current_depth: usize, target_depth: usize) {
            let _span = spf!(self.spanner => "Span({current_depth})");

            if current_depth >= target_depth {
                return;
            }
            return self.helper_macro_3(current_depth + 1, target_depth);
        }

        fn helper_macro_4(&self, current_depth: usize, target_depth: usize) {
            let _span = spf!(self.spanner, "Span({})", current_depth);

            if current_depth >= target_depth {
                return;
            }
            return self.helper_macro_2(current_depth + 1, target_depth);
        }
    }

    #[test]
    fn it_works() {
        let helper = Helper {
            spanner: VecSpanner::new(),
        };

        let expected = r#"┌Span(0)
|  Span(1)
|   ┌Span(2)
|   ┆  Span(3)
|   ┆   ┌Span(4)
|   ┆   |  Span(5)
|   ┆   |  Span(5)
|   ┆   └Span(4)
|   ┆  Span(3)
|   └Span(2)
|  Span(1)
└Span(0)
"#;

        helper.helper(0, 5);
        let vec = helper.spanner.writer.into_inner().unwrap();
        assert_eq!(expected.bytes().collect::<Vec<_>>(), vec);

        let helper = Helper {
            spanner: VecSpanner::new(),
        };
        helper.helper_macro(0, 5);
        let vec = helper.spanner.writer.into_inner().unwrap();
        assert_eq!(expected.bytes().collect::<Vec<_>>(), vec)
    }

    #[test]
    fn test_config() {
        let helper = Helper {
            spanner: VecSpanner::new().with_config(Config::new().with_skip(3)),
        };

        let expected = r#"┌Span(0)
|  Span(1)
|    Span(2)
|     ┌Span(3)
|     ┊  Span(4)
|     ┊    Span(5)
|     ┊    Span(5)
|     ┊  Span(4)
|     └Span(3)
|    Span(2)
|  Span(1)
└Span(0)
"#;

        helper.helper(0, 5);
        let vec = helper.spanner.writer.into_inner().unwrap();
        assert_eq!(expected.bytes().collect::<Vec<_>>(), vec)
    }

    #[test]
    fn test_level() {
        let mut writer = Cursor::new(Vec::new());
        let spanner = Spanner::from_writer(&mut writer)
            .with_config(Config::new().with_skip(3).with_level(Level::Info));

        {
            let _span = spanner.enter_with_level(Level::Debug, "foo");
            // Span is dropped here
        }

        assert_eq!(writer.get_ref(), &[]);
    }

    #[test]
    fn test_skip() {
        let helper = Helper {
            spanner: VecSpanner::new().with_config(Config::new().with_skip(0)),
        };

        let expected = r#" Span(0)
   Span(1)
     Span(2)
       Span(3)
         Span(4)
           Span(5)
           Span(5)
         Span(4)
       Span(3)
     Span(2)
   Span(1)
 Span(0)
"#;

        helper.helper(0, 5);
        let vec = helper.spanner.writer.into_inner().unwrap();
        assert_eq!(expected.bytes().collect::<Vec<_>>(), vec)
    }

    #[test]
    fn it_works_macros() {
        let expected = r#"┌Span(0)
|  Span(1)
|   ┌Span(2)
|   ┆  Span(3)
|   ┆   ┌Span(4)
|   ┆   |  Span(5)
|   ┆   |  Span(5)
|   ┆   └Span(4)
|   ┆  Span(3)
|   └Span(2)
|  Span(1)
└Span(0)
"#;
        let expected_3 = r#"┌Span({current_depth})
|  Span({current_depth})
|   ┌Span({current_depth})
|   ┆  Span({current_depth})
|   ┆   ┌Span({current_depth})
|   ┆   |  Span({current_depth})
|   ┆   |  Span({current_depth})
|   ┆   └Span({current_depth})
|   ┆  Span({current_depth})
|   └Span({current_depth})
|  Span({current_depth})
└Span({current_depth})
"#;

        let helper = Helper {
            spanner: VecSpanner::new(),
        };
        helper.helper_macro(0, 5);
        let vec = helper.spanner.writer.into_inner().unwrap();
        assert_eq!(expected.bytes().collect::<Vec<_>>(), vec);

        let helper = Helper {
            spanner: VecSpanner::new(),
        };
        helper.helper_macro_2(0, 5);
        let vec = helper.spanner.writer.into_inner().unwrap();
        assert_eq!(expected.bytes().collect::<Vec<_>>(), vec);

        let helper = Helper {
            spanner: VecSpanner::new(),
        };
        helper.helper_macro_3(0, 5);
        let vec = helper.spanner.writer.into_inner().unwrap();
        assert_eq!(expected_3.bytes().collect::<Vec<_>>(), vec);

        let helper = Helper {
            spanner: VecSpanner::new(),
        };
        helper.helper_macro_4(0, 5);
        let vec = helper.spanner.writer.into_inner().unwrap();
        assert_eq!(expected.bytes().collect::<Vec<_>>(), vec);
    }
}
