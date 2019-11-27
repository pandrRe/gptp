pub trait ToRange {
    fn to_range(&self) -> std::ops::Range<usize>;
}

impl ToRange for (usize, usize) {
    fn to_range(&self) -> std::ops::Range<usize> {
        std::ops::Range {
            start: self.0,
            end: self.1
        }
    }
}