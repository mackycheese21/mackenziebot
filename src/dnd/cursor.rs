#[derive(Clone, Copy, Debug)]
pub struct Cursor<'a> {
    value: &'a str,
    pub index: usize,
}

impl Cursor<'_> {
    pub fn next(&self) -> Result<(Self, char), usize> {
        self.value.chars().nth(self.index).map(|ch| (Cursor { value: self.value, index: self.index + 1 }, ch)).ok_or(self.index)
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
        while let Some(ch) = self.value.chars().nth(self.index) {
            if ch.is_whitespace() {
                self.index += 1
            } else {
                break;
            }
        }
    }
    pub fn new(value: &str) -> Cursor {
        Cursor { value, index: 0 }
    }
}
