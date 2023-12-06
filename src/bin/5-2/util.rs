use std::{collections::HashMap, ops::Range};

pub fn add_filter(map: &mut HashMap<Range<usize>, i64>, range: &mut Range<usize>, offset: i64) {
    for other_range in map.clone().keys() {
        match range.intersects(other_range) {
            Overlap::IsContained => {
                panic!("range conflict: {range:?} is contained in {other_range:?}")
            }
            Overlap::None => {}
            Overlap::PartialLower => {
                range.end = other_range.start;
                break;
            }
            Overlap::PartialUpper => {
                range.start = other_range.end;
                break;
            }
            Overlap::Contains => {
                let lower = Range {
                    start: range.start,
                    end: other_range.start,
                };
                map.insert(lower, offset);
                let upper = Range {
                    start: other_range.end,
                    end: range.end,
                };
                map.insert(upper, offset);
                return;
            }
        };
    }
    map.insert(range.to_owned(), offset);
}

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

pub fn map_range(
    seed: &Range<usize>,
    map: &Range<usize>,
    offset: i64,
) -> (Option<Range<usize>>, Vec<Range<usize>>) {
    let mut mapped_range = None;
    let mut new_ranges = vec![];
    match seed.intersects(map) {
        Overlap::IsContained => {
            print!("contained = ");
            let start = (seed.start as i64 + offset) as usize;
            let end = (seed.end as i64 + offset) as usize;
            mapped_range = Some(Range { start, end })
        }
        Overlap::PartialUpper => {
            print!("partialupper = ");
            mapped_range = Some(Range {
                start: (seed.start as i64 + offset) as usize,
                end: (map.end as i64 + offset) as usize,
            });
            new_ranges.push(Range {
                start: map.end,
                end: seed.end,
            });
        }
        Overlap::PartialLower => {
            print!("partiallower = ");
            mapped_range = Some(Range {
                start: (map.start as i64 + offset) as usize,
                end: (seed.end as i64 + offset) as usize,
            });
            new_ranges.push(Range {
                start: seed.start,
                end: map.start,
            });
        }
        Overlap::Contains => {
            print!("contains = ");
            mapped_range = Some(Range {
                start: (map.start as i64 + offset) as usize,
                end: (seed.end as i64 + offset) as usize,
            });
            new_ranges.push(Range {
                start: map.end,
                end: seed.end,
            });
            new_ranges.push(Range {
                start: seed.start,
                end: map.start,
            });
        }
        Overlap::None => {
            print!("none = ");
        }
    };
    println!("{mapped_range:?}, new: {new_ranges:?}");
    (mapped_range, new_ranges)
}