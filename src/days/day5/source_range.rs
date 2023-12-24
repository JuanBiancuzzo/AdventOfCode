use std::cmp::Ordering;

#[derive(Debug, Clone, Copy, Eq)]
pub struct SourceRange {
    pub start: u32,
    pub range: u32,
}

impl SourceRange {
    pub fn new(start: u32, range: u32) -> Option<Self> {
        if range == 0 {
            return None;
        }
        Some(Self { start, range })
    }

    pub fn get_min(&self) -> u32 {
        self.start
    }
}

impl PartialOrd for SourceRange {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.start.partial_cmp(&other.start)
    }
}

impl Ord for SourceRange {
    fn cmp(&self, other: &Self) -> Ordering {
        self.start.cmp(&other.start)
    }
}

impl PartialEq for SourceRange {
    fn eq(&self, other: &Self) -> bool {
        self.start == other.start
    }
}