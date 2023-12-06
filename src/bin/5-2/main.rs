use std::{collections::HashMap, ops::Range};
use util::*;

mod tests;
mod util;

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
    let maps: Vec<HashMap<Range<usize>, i64>> = input_blocks
        .map(|m| {
            let mut map: HashMap<Range<usize>, i64> = HashMap::new();
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
            let mut found = handle_stage(&mut todo, stage);
            todo.append(&mut found);
            todo.dedup();
            println!("---");
        }
        res_ranges.append(&mut todo);
        println!("===");
    }
    res_ranges.sort_by(|a,b|a.start.cmp(&b.start));
    res_ranges[0].start
}

fn handle_stage(todo: &mut Vec<Range<usize>>, map: &HashMap<Range<usize>, i64>) -> Vec<Range<usize>> {
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




