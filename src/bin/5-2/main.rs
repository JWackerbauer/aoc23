use std::ops::Range;
use lib::_5::*;

#[cfg(test)]
mod tests;

fn main() {
    let input = std::fs::read_to_string("input/5.txt").unwrap();
    println!("{}", filter_through(input));
}

pub fn filter_through(input: String) -> usize {
    let mut input_blocks = input.split("\n\n");
    let mut seeds_raw = input_blocks
        .next()
        .expect("no seeds")
        .split(" ")
        .filter_map(|n| n.parse::<usize>().ok());
    let mut seed_ranges = vec![];
    while let Some(start) = seeds_raw.next() {
        seed_ranges.push(Range {
            start,
            end: start + seeds_raw.next().expect("no range"),
        })
    }
    let maps: Vec<Vec<(Range<usize>, i64)>> = input_blocks
        .map(|m| {
            let mut map: Vec<(Range<usize>, i64)> = vec![];
            for mut n in m.lines().filter_map(|l| {
                let mut numbers = l.split(" ").filter_map(|n| n.parse::<usize>().ok());
                if numbers.clone().count() != 3 {
                    return None;
                };
                let dest = numbers.next().expect("no dest");
                let src = numbers.next().expect("no src");
                let amt = numbers.next().expect("no amt");
                let offset = dest as i64 - src as i64;
                Some((
                    Range {
                        start: src,
                        end: src + amt,
                    },
                    offset,
                ))
            }) {
                add_filter(&mut map, &mut n.0, n.1);
            }
            map
        })
        .collect();
    let mut res_ranges = vec![];
    for start in seed_ranges {
        let mut todo = vec![start];
        for stage in maps.as_slice() {
            todo = handle_stage(&mut todo, stage);
            println!("---");
        }
        res_ranges.append(&mut todo);
        println!("===");
    }
    res_ranges.sort_by(|a,b|a.start.cmp(&b.start));
    res_ranges[0].start
}

fn handle_stage(todo: &mut Vec<Range<usize>>, map: &Vec<(Range<usize>, i64)>) -> Vec<Range<usize>> {
    let mut found_ranges = vec![];
    println!("Todo: {todo:?}");
    println!("Stage: {map:?}");
    while let Some(range) = todo.pop() {
        let mut mapped = None;
        for (_map_range, offset) in map {
            println!("Current: {range:?}, remaining: {todo:?}");
            println!("{_map_range:?} + ({offset})");
            let mut res = map_range(&range, _map_range, *offset);
            mapped = res.0;
            if mapped.is_some() {
                todo.append(&mut res.1);
                break;
            }
        } 
        if let Some(mapped) = mapped {
            found_ranges.push(mapped);
        } else {
            found_ranges.push(range);
        }
    }
    println!("Done; found: {found_ranges:?}");
    found_ranges
}

fn map_range(
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

fn add_filter(map: &mut Vec<(Range<usize>, i64)>, range: &mut Range<usize>, offset: i64) {
    for other_range in map.clone().iter().map(|i|&i.0) {
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
                map.push((lower, offset));
                let upper = Range {
                    start: other_range.end,
                    end: range.end,
                };
                map.push((upper, offset));
                return;
            }
        };
    }
    map.push((range.to_owned(), offset));
}