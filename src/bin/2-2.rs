use std::collections::HashMap;

fn main() {
    let input = std::fs::read_to_string("input/2.txt").unwrap();
    println!(
        "{}",
        input
            .lines()
            .map(|l| {
                let mut line = l.split(": ");
                let game = line
                    .next()
                    .expect("no game")
                    .split(" ")
                    .last()
                    .expect("no game number")
                    .parse::<u64>()
                    .expect("couldn't parse number");
                println!("Game {game}: ");
                let mut game_outcomes: Vec<HashMap<&str, u64>> = vec![];
                for outcome in line.next().expect("no content").split("; ") {
                    let mut game_res: HashMap<&str, u64> = HashMap::new();
                    for mut res in outcome.split(", ").map(|r| r.split(" ")) {
                        let num = res
                            .next()
                            .expect("no num")
                            .parse::<u64>()
                            .expect("couldn't parse number");
                        let color = res.next().expect("no color");
                        match game_res.get(color) {
                            Some(n) => {
                                game_res.insert(color, n + num);
                            }
                            None => {
                                game_res.insert(color, num);
                            }
                        };
                        print!("{} {} ",num, color);
                    }
                    game_outcomes.push(game_res);
                    println!("");
                }
                let mut least_colors: HashMap<&str, u64> = HashMap::new();
                for game_res in game_outcomes {
                    for (k,v) in game_res {
                        match least_colors.get(k) {
                            Some(c) => {
                                if v > *c {
                                    least_colors.insert(k, v);
                                }
                            },
                            None => {least_colors.insert(k, v);}
                        }
                    }
                }
                let mut result = 1u64;
                for (k,v) in least_colors {
                    print!("{v} {k} ");
                    result *= v;
                }
                print!("= {result}");
                println!("");
                result
            })
            .sum::<u64>()
    );
}

