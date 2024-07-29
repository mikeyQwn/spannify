pub struct Config {
    pub tabwidth: usize,
}

impl Config {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Default for Config {
    fn default() -> Self {
        Self { tabwidth: 2 }
    }
}
