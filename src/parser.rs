struct Parser {
    pos: usize,
    input: String,
}

impl Parser {
    fn end_of_line(&self) -> bool {
        self.pos >= self.input.len()
    }

    fn starts_with(&self, s: &str) -> bool {
        self.input[self.pos..].starts_with(s)
    }
}
