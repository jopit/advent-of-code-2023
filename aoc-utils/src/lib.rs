use std::str::from_utf8;

pub struct Tokenizer<'a> {
    chars: &'a [u8],
    index: usize,
}

impl<'a> Tokenizer<'a> {
    pub fn new(input: &'a str) -> Self {
        Tokenizer {
            chars: input.as_bytes(),
            index: 0,
        }
    }

    pub fn consume(&mut self, expected: &str) {
        if let Some(token) = self.next() {
            if token != expected {
                panic!(r#"expected token: "{expected}", got "{token}""#)
            }
        } else {
            panic!(r#"no more tokens, expected token: "{expected}""#)
        }
    }

    #[inline(always)]
    pub fn get(&mut self) -> &'a str {
        self.next().unwrap()
    }

    #[inline(always)]
    fn char(&self) -> u8 {
        self.chars[self.index]
    }
}

impl<'a> Iterator for Tokenizer<'a> {
    type Item = &'a str;

    fn next(&mut self) -> Option<Self::Item> {
        let len = self.chars.len();

        // Strip leading whitespace
        while self.index < len && self.char().is_ascii_whitespace() {
            self.index += 1;
        }
        if self.index == len {
            return None;
        }

        let start = self.index;

        if !self.char().is_ascii_alphanumeric() {
            self.index += 1;
            let ret = from_utf8(&self.chars[start..self.index]).unwrap();
            return Some(ret);
        }

        while self.index < len && self.char().is_ascii_alphanumeric() {
            self.index += 1;
        }
        if start == self.index {
            None
        } else {
            let ret = from_utf8(&self.chars[start..self.index]).unwrap();
            Some(ret)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_string() {
        let mut tokens = Tokenizer::new("");
        assert_eq!(tokens.next(), None);

        let mut tokens = Tokenizer::new("    ");
        assert_eq!(tokens.next(), None);
    }

    #[test]
    fn test_one_word() {
        let mut tokens = Tokenizer::new(" abc ");
        assert_eq!(tokens.next(), Some("abc"));
        assert_eq!(tokens.next(), None);
    }

    #[test]
    fn test_multiple_words() {
        let mut tokens = Tokenizer::new(" abc  def  ghi 123");
        assert_eq!(tokens.next(), Some("abc"));
        assert_eq!(tokens.next(), Some("def"));
        assert_eq!(tokens.next(), Some("ghi"));
        assert_eq!(tokens.next(), Some("123"));
        assert_eq!(tokens.next(), None);
    }

    #[test]
    fn test_words_and_symbols() {
        let mut tokens = Tokenizer::new("abc: def = (123, 456)");
        assert_eq!(tokens.next(), Some("abc"));
        assert_eq!(tokens.next(), Some(":"));
        assert_eq!(tokens.next(), Some("def"));
        assert_eq!(tokens.next(), Some("="));
        assert_eq!(tokens.next(), Some("("));
        assert_eq!(tokens.next(), Some("123"));
        assert_eq!(tokens.next(), Some(","));
        assert_eq!(tokens.next(), Some("456"));
        assert_eq!(tokens.next(), Some(")"));
        assert_eq!(tokens.next(), None);
    }

    #[test]
    fn test_consume() {
        let mut tokens = Tokenizer::new("abc: def = (123, 456)");
        assert_eq!(tokens.next(), Some("abc"));
        tokens.consume(":");
        assert_eq!(tokens.next(), Some("def"));
        tokens.consume("=");
        tokens.consume("(");
        assert_eq!(tokens.next(), Some("123"));
        tokens.consume(",");
        assert_eq!(tokens.next(), Some("456"));
        tokens.consume(")");
        assert_eq!(tokens.next(), None);
    }
}
