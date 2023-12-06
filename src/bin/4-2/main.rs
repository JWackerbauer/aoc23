fn main() {
    let input = std::fs::read_to_string("input/4.txt").unwrap();

//     let input = r#"Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
// Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
// Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
// Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
// Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
// Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11"#;

    let data = input
        .lines()
        .map(|l| {
            let mut content = l.split(": ").last().expect("no content").split(" | ");
            let winning_nums: Vec<u64> = content
                .next()
                .expect("no winning_nums")
                .split(" ")
                .filter_map(|c| c.parse::<u64>().ok())
                .collect();
            content
                .next()
                .expect("no my_nums")
                .split(" ")
                .filter_map(|c| c.parse::<u64>().ok())
                .filter(|n| winning_nums.contains(n))
                .count()
        })
        .collect::<Vec<usize>>();
    let mut result = 0;
    for i in 0..data.len() {
        result += process_card(i, &data);
    }
    println!("{result}",)
}

fn process_card(i: usize, cards: &Vec<usize>) -> usize {
    let Some(card) = cards.get(i) else {
        return 0;
    };
    let mut copies = 1;
    for x in 1..*card + 1 {
        copies += process_card(i + x, cards);
    }
    copies
}
