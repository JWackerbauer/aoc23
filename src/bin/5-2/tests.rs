use std::ops::Range;

use crate::*;

#[test]
fn test_intersects() {
    let a: Range<usize> = 1..3;
    let b: Range<usize> = 3..5;
    let c: Range<usize> = 2..4;
    let d: Range<usize> = 1..5;
    assert_eq!(a.intersects(&b), Overlap::None);
    assert_eq!(c.intersects(&a), Overlap::PartialUpper);
    assert_eq!(c.intersects(&b), Overlap::PartialLower);
    assert_eq!(d.intersects(&a), Overlap::PartialUpper);
    assert_eq!(d.intersects(&b), Overlap::PartialLower);
    assert_eq!(d.intersects(&c), Overlap::Contains);
    assert_eq!(a.intersects(&d), Overlap::IsContained);
    assert_eq!(b.intersects(&d), Overlap::IsContained);
    assert_eq!(c.intersects(&d), Overlap::IsContained);
}

#[test]
fn test_input() {
    let input = 
r#"seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4
"#;
    assert_eq!(filter_through(input.to_string()), 46);
}