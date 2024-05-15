pub struct LinesIndices<'a> {
    str: &'a str,
    last_index: usize,
    char_indices: std::str::CharIndices<'a>,
}

impl<'a> LinesIndices<'a> {
    pub fn new(str: &'a str) -> Self {
        Self {
            str,
            last_index: 0,
            char_indices: str.char_indices(),
        }
    }
}

impl<'a> Iterator for LinesIndices<'a> {
    type Item = (usize, &'a str);

    fn next(&mut self) -> Option<Self::Item> {
        let (i, _) = self.char_indices.by_ref().find(|item| item.1 == '\n')?;
        let current_index = self.last_index;
        self.last_index = i + 1;

        Some((current_index, &self.str[current_index..i]))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn yields_indices() {
        let mut lines = LinesIndices::new("hello\nworld\n");
        assert_eq!(lines.next(), Some((0, "hello")));
        assert_eq!(lines.next(), Some((6, "world")));
    }
}

