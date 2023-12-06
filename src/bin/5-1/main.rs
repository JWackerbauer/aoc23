fn main() {
//     let input = r#"seeds: 79 14 55 13

// seed-to-soil map:
// 50 98 2
// 52 50 48

// soil-to-fertilizer map:
// 0 15 37
// 37 52 2
// 39 0 15

// fertilizer-to-water map:
// 49 53 8
// 0 11 42
// 42 0 7
// 57 7 4

// water-to-light map:
// 88 18 7
// 18 25 70

// light-to-temperature map:
// 45 77 23
// 81 45 19
// 68 64 13

// temperature-to-humidity map:
// 0 69 1
// 1 0 69

// humidity-to-location map:
// 60 56 37
// 56 93 4"#;
    let input = std::fs::read_to_string("input/5.txt").unwrap();
    let mut input_blocks = input.split("\n\n");
    let seeds = input_blocks
        .next()
        .expect("no seeds")
        .split(" ")
        .filter_map(|n| n.parse::<usize>().ok());
    let maps: Vec<Vec<Map>> = input_blocks
        .map(|m| {
            m.lines()
                .filter_map(|l| {
                    let mut numbers = l.split(" ").filter_map(|n| n.parse::<usize>().ok());
                    if numbers.clone().count() != 3 {
                        return None;
                    };
                    Some(Map {
                        destination: numbers.next().expect("no dest"),
                        source: numbers.next().expect("no src"),
                        amount: numbers.next().expect("no amount"),
                    })
                })
                .collect::<Vec<Map>>()
        })
        .filter(|v| v.len() > 0)
        .collect();
    let mut lowest: usize = usize::MAX;
    for mut seed in seeds {
        print!("Seed: {seed} ");
        for stage in maps.as_slice() {
            for map in stage {
                let Some(offset) = seed.checked_sub(map.source) else {
                    // Below in source range
                    continue;
                };
                if offset > map.amount {
                    // Above source range
                    continue;
                } else {
                    seed = map.destination + offset;
                    break;
                }
            }
            print!("-> {seed} ");
        }
        println!("");
        if seed < lowest {
            lowest = seed;
        }
    }
    println!("lowest: {lowest}");
}

#[derive(Debug)]
struct Map {
    destination: usize,
    source: usize,
    amount: usize,
}
