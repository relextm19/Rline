use std::fmt;
use std::ops::AddAssign;

#[derive(Clone, Debug)]
pub struct Data {
    pub name: String,
    pub line_count: usize,
    pub character_count: usize,
    pub whitespace_count: usize,
}

impl fmt::Display for Data {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
           "{:<25}: [line_count: {:>5} | character_count: {:>10} | whitespace_count: {:>5}]",
            self.name, self.line_count, self.character_count, self.whitespace_count
        )
    }
}

impl AddAssign for Data {
    fn add_assign(&mut self, other: Self) {
        self.line_count += other.line_count;
        self.character_count += other.character_count;
        self.whitespace_count += other.whitespace_count;
    }
}