//! Configuration for span generators

use crate::level::Level;

/// Config that determines the ouput of the span generator
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct Config<T = fn(usize) -> char>
where
    T: Fn(usize) -> char,
{
    /// Determines how many spaces is added per depth. Default is `2`.
    pub tabwidth: usize,
    /// Determines the frequency of vertical bars. `skip: 2` means a bar is placed every 2 spans. The skip value of 0 means that no bars are displayed. Default is `2`.
    pub skip: usize,
    /// Function that maps the depth of a span to a vertical bar character.
    pub depthmap: T,
    /// Determines the minumum level for the spans. Spans with level below the `level` are
    /// ignored. Deafult is `Level::Info`
    pub level: Level,
}

impl Config {
    /// Creates a `Config` instance with default values
    ///
    /// # Defaults
    /// - `tabwidth`: 2
    /// - `skip`: 2
    /// - `depthmap`: A function that cycles through `['|', '¦', '┆', '┊']`
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }
}

impl<T> Config<T>
where
    T: Fn(usize) -> char,
{
    /// Replaces the function that maps depth to a character.
    ///
    /// # Parameters
    /// - `depthmap`: A new function to map depth to a character.
    ///
    /// # Examples
    /// ```
    /// use spannify::config::Config;
    ///
    /// let config = Config::new().with_depthmap(|depth| if depth % 2 == 0 { '|' } else { '¦' });
    /// ```
    #[must_use]
    pub fn with_depthmap<U: Fn(usize) -> char>(self, depthmap: U) -> Config<U> {
        Config {
            tabwidth: self.tabwidth,
            skip: self.skip,
            depthmap,
            level: self.level,
        }
    }

    /// Replaces the skip value.
    ///
    /// # Parameters
    /// - `skip`: The new frequency for vertical bars.
    ///
    /// # Examples
    /// ```
    /// use spannify::config::Config;
    ///
    /// let config = Config::new().with_skip(4);
    /// ```
    #[must_use]
    pub fn with_skip(self, skip: usize) -> Self {
        Self { skip, ..self }
    }

    /// Replaces the level value.
    ///
    /// # Parameters
    /// - `level`: The new minimum level for the spans.
    ///
    /// # Examples
    /// ```
    /// use spannify::config::Config;
    ///
    /// let config = Config::new().with_skip(4);
    /// ```
    #[must_use]
    pub fn with_level(self, level: Level) -> Self {
        Self { level, ..self }
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            tabwidth: 2,
            skip: 2,
            depthmap: default_depthmap,
            level: Level::Info,
        }
    }
}
/// Default function that maps depth to a vertical bar character.
///
/// Cycles through the characters `['|', '¦', '┆', '┊']` based on depth.
fn default_depthmap(depth: usize) -> char {
    const DEPTHMAP: [char; 4] = ['|', '¦', '┆', '┊'];
    DEPTHMAP[depth % DEPTHMAP.len()]
}
