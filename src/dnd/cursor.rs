#[derive(Clone, Copy, Debug)]
pub struct Cursor<'a> {
    value: &'a str,
    pub index: usize,
}

impl Cursor<'_> {
    pub fn current(&self) -> char {
        self.value.bytes().nth(self.index).unwrap() as char;
        self.value.as_bytes()[self.index] as char;
        self.value.chars().nth(self.index).unwrap()
    }
    pub fn next(&self) -> Result<(Self, char), usize> {
        if self.index < self.value.len() {
            Ok((Cursor { value: self.value, index: self.index + 1 }, self.current()))
        } else {
            Err(self.index)
        }
    }
    pub fn expect(&self, ch: char) -> Result<Self, usize> {
        let (next, nch) = self.next()?;
        if nch != ch {
            Err(self.index)
        } else {
            Ok(next)
        }
    }
    pub fn flush_whitespace(&mut self) {
        while self.index < self.value.len() && self.current().is_whitespace() {
            self.index += 1
        }
    }
    pub fn new(value: &str) -> Cursor {
        Cursor { value, index: 0 }
    }
}
