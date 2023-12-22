use std::char;

pub trait StringOps {
    fn index_of_char(&self, c: char) -> Option<usize>;
}

impl StringOps for Vec<char> {
    fn index_of_char(&self, c: char) -> Option<usize> {
        let mut index: usize = 0;
        for cc in self.iter() {
            if *cc == c {
                return Some(index);
            }
            index += 1;
        }
        return None;
    }
}