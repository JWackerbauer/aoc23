use std::ops::Range;

impl IntervalRange for Range<usize> {
    fn intersects(&self, other: &Self) -> Overlap {
        match (
            (other.start < self.end && self.end <= other.end),
            (other.start <= self.start && self.start < other.end),
        ) {
            (true, true) => Overlap::IsContained,
            (true, false) => Overlap::PartialLower,
            (false, true) => Overlap::PartialUpper,
            (false, false) => {
                if self.start < other.start && self.end >= other.end {
                    Overlap::Contains
                } else {
                    Overlap::None
                }
            }
        }
    }
}
pub trait IntervalRange {
    fn intersects(&self, other: &Self) -> Overlap;
}

#[derive(PartialEq, Debug)]
pub enum Overlap {
    Contains,
    IsContained,
    PartialLower,
    PartialUpper,
    None,
}