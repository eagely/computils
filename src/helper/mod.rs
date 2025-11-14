pub trait StrExt {
    fn drop_lines(&self, n: usize) -> &str;
    fn take_lines(&self, n: usize) -> &str;
    fn sdnl(&self) -> impl Iterator<Item = &str>;
}

impl StrExt for &str {
    fn drop_lines(&self, n: usize) -> &str {
        let mut lines_to_skip = n;
        for (i, c) in self.char_indices() {
            if c == '\n' {
                lines_to_skip -= 1;
                if lines_to_skip == 0 {
                    return &self[i + 1..];
                }
            }
        }
        ""
    }

    fn take_lines(&self, n: usize) -> &str {
        if n == 0 {
            return "";
        }
        let mut lines_remaining = n;
        for (i, c) in self.char_indices() {
            if c == '\n' {
                lines_remaining -= 1;
                if lines_remaining == 0 {
                    return &self[..i];
                }
            }
        }
        self
    }

    fn sdnl(&self) -> impl Iterator<Item = &str> {
        self.split("\n\n")
    }
}
