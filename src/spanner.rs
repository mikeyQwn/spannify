use core::cell::RefCell;

pub struct Spanner<T>
where
    T: std::io::Write,
{
    writer: RefCell<T>,
}

impl<T> Spanner<T>
where
    T: std::io::Write,
{
    pub fn from_writer(writer: T) -> Self {
        Self {
            writer: RefCell::new(writer),
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
        }
    }
}

impl Default for VecSpanner {
    fn default() -> Self {
        Self {
            writer: RefCell::new(Vec::new()),
        }
    }
}

pub type FileSpanner = Spanner<std::fs::File>;

impl FileSpanner {
    pub fn new(file: std::fs::File) -> Self {
        Self {
            writer: RefCell::new(file),
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
        }
    }
}
