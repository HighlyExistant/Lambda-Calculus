use std::ops::Range;
#[derive(Debug, Clone, PartialEq)]
pub struct Span {
    pub range: Range<usize>,
}

impl Span {
    pub fn new(range: Range<usize>) -> Self {
        Self { range }
    }
}