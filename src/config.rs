//! Configuration for span generators

/// Config that determines the ouput of the span generator
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct Config<T = fn(usize) -> char>
where
    T: Fn(usize) -> char,
{
    /// Determines how many spaces is added per depth. Default is `2`.
    pub tabwidth: usize,
    /// Determines the frequency of vertical bars. `skip: 2` means a bar is placed every 2 spans. Default is `2`.
    pub skip: usize,
    /// Function that maps the depth of a span to a vertical bar character.
    pub depthmap: T,
}

impl Config {
    /// Creates a `Config` instance with default values
    ///
    /// # Defaults
    /// - `tabwidth`: 2
    /// - `skip`: 2
    /// - `depthmap`: A function that cycles through `['|', '¦', '┆', '┊']`
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
    /// use spanner::config::Config;
    ///
    /// let config = Config::new().with_depthmap(|depth| if depth % 2 == 0 { '|' } else { '¦' });
    /// ```
    pub fn with_depthmap<U: Fn(usize) -> char>(self, depthmap: U) -> Config<U> {
        Config {
            tabwidth: self.tabwidth,
            skip: self.skip,
            depthmap,
        }
    }

    /// Replaces the skip value.
    ///
    /// # Parameters
    /// - `skip`: The new frequency for vertical bars.
    ///
    /// # Examples
    /// ```
    /// use spanner::config::Config;
    ///
    /// let config = Config::new().with_skip(4);
    /// ```
    pub fn with_skip(self, skip: usize) -> Self {
        Self {
            tabwidth: self.tabwidth,
            skip,
            depthmap: self.depthmap,
        }
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            tabwidth: 2,
            skip: 2,
            depthmap: default_depthmap,
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
