fn main() {
    let input = std::fs::read_to_string("input/1.txt").unwrap();
    println!(
        "{}",
        input
            .lines()
            .map(|l| {
                let mut iter = l.chars().filter(|c| c.is_numeric()).peekable();
                format!("{}{}", iter.peek().unwrap().clone(), iter.last().unwrap())
                    .parse::<u64>()
                    .unwrap()
            })
            .sum::<u64>()
    )
}
