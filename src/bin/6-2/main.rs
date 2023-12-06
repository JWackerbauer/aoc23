use std::usize;
use lib::_6::*;


#[test]
fn test_input() {
    let input = 
r#"Time:      7  15   30
Distance:  9  40  200"#;
    assert_eq!(do_stuff(input.to_string()), 71503);
}

fn main() {
    let input = std::fs::read_to_string("input/6.txt").unwrap();
    println!("{}", do_stuff(input));
}

fn do_stuff(input: String) -> usize {
    let item = parse_input(input);
    ways_to_win(item.0, item.1)
}

fn parse_input(input: String) -> (usize, usize) {
    let mut lines = input.lines();
    let times: String = lines.next().expect("no times").split(" ").filter_map(|s|s.parse::<usize>().ok()).flat_map(|c|c.to_string().chars().collect::<Vec<char>>()).collect();
    let distances: String = lines.next().expect("no distance").split(" ").filter_map(|s|s.parse::<usize>().ok()).flat_map(|c|c.to_string().chars().collect::<Vec<char>>()).collect();
    (times.parse::<usize>().expect("can't parse time"), distances.parse::<usize>().expect("can't parse distance"))
}


