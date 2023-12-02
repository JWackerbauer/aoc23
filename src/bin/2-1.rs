use std::collections::HashMap;

fn main() {
//     let input = r#"Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
// Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
// Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
// Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
// Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"#;
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
                let mut possible = true;
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
                    possible = is_game_possible(game_res);
                    if !possible {
                        break;
                    }
                }
                match possible {
                    true => game,
                    false => 0u64,
                }
            })
            .sum::<u64>()
    );
}

fn is_game_possible(game: HashMap<&str, u64>) -> bool {
    let mut possible = true;
    if let Some(red) = game.get("red") {
        print!("red: {red} ");
        if red > &12u64 {
            possible = false;
        }
    }
    if let Some(green) = game.get("green") {
        print!("green: {green} ");
        if green > &13u64 {
            possible = false;
        }
    }
    if let Some(blue) = game.get("blue") {
        print!("blue: {blue} ");
        if blue > &14u64 {
            possible = false;
        }
    }
    println!("-> {possible}");
    possible
}
