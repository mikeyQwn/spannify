pub struct Config<T = fn(usize) -> char>
where
    T: Fn(usize) -> char,
{
    pub tabwidth: usize,
    pub skip: usize,
    pub depthmap: T,
}

impl Config {
    pub fn new() -> Self {
        Self::default()
    }
}

impl<T> Config<T>
where
    T: Fn(usize) -> char,
{
    pub fn with_depthmap<U: Fn(usize) -> char>(self, depthmap: U) -> Config<U> {
        Config {
            tabwidth: self.tabwidth,
            skip: self.skip,
            depthmap,
        }
    }

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

fn default_depthmap(depth: usize) -> char {
    const DEPTHMAP: [char; 4] = ['|', '¦', '┆', '┊'];
    DEPTHMAP[depth % DEPTHMAP.len()]
}
