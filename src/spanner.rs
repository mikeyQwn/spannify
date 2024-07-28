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

    pub fn enter_span(&self, name: &str) -> Span<T> {
        Span::enter(&self, name)
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
        let (enter_message, drop_message) = Self::generate_messages(name);

        let _ = parent.writer.borrow_mut().write(enter_message.as_ref());
        Self {
            parent,
            drop_message,
        }
    }

    fn generate_messages(name: &str) -> (String, String) {
        let enter_message = format!("{} entered\n", name);
        let drop_message = format!("{} dropped\n", name);
        (enter_message, drop_message)
    }
}

impl<'a, T> Drop for Span<'a, T>
where
    T: std::io::Write,
{
    fn drop(&mut self) {
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
