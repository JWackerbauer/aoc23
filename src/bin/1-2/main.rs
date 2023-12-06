fn main() {
    let input = std::fs::read_to_string("input/1.txt").unwrap();
    let digits = vec![
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];
    println!(
        "{}",
        input
            .lines()
            .map(|l| {
                print!("{l} ");
                let mut out = l.to_string();
                for (n, s) in digits.iter().enumerate() {
                    for (i, _m) in l.to_string().match_indices(s) {
                        out.replace_range(i..(i + 1), (n + 1).to_string().as_str());
                    }
                }
                print!("-> {out}");
                out
            })
            .map(|l| {
                let mut iter = l.chars().filter(|c| c.is_numeric()).peekable();
                let res = format!("{}{}", iter.peek().unwrap().clone(), iter.last().unwrap())
                    .parse::<u64>()
                    .unwrap();
                println!(" = {res}");
                res
            })
            .sum::<u64>()
    )
}
