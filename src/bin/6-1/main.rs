use lib::_6::*;

#[test]
fn test_input() {
    let input = 
r#"Time:      7  15   30
Distance:  9  40  200"#;
    assert_eq!(do_stuff(input.to_string()), 288);
}

fn main() {
    let input = std::fs::read_to_string("input/6.txt").unwrap();
    println!("{}", do_stuff(input));
}

fn do_stuff(input: String) -> usize {
    let mut out = 1;
    for item in parse_input(input) {
        out *= ways_to_win(item.0, item.1);
    }
    out
}

fn parse_input(input: String) -> Vec<(usize, usize)> {
    let mut lines = input.lines();
    let mut times = lines.next().expect("no times").split(" ").filter_map(|s|s.parse::<usize>().ok());
    let mut distances = lines.next().expect("no distance").split(" ").filter_map(|s|s.parse::<usize>().ok());
    let mut res = vec![];
    while let Some(time) = times.next() {
        let Some(distance) = distances.next() else {
            panic!("more distances than times");
        };
        res.push((time,distance));
    }
    if distances.count() > 0 {
        panic!("more times than distances");
    }
    res
}

