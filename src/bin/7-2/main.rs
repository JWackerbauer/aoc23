use std::{cmp::Ordering, collections::HashMap, u16, usize};

#[cfg(test)]
mod tests;

pub fn main() {
    let input = std::fs::read_to_string("input/7.txt").unwrap();
    println!("{}",get_winnings(input));
}

fn get_winnings(input: String) -> usize {
    let mut games = parse_input(input);
    games = rank_games(&mut games);
    let mut winnings = 0;
    for (i, game) in games.iter().enumerate() {
        println!("{} * {}", game.1, (i + 1));
        winnings += game.1 * (i + 1)
    }
    winnings
}

fn compare_hands(a: &Hand, b: &Hand) -> Ordering {
    match score(a).cmp(&score(b)) {
        Ordering::Equal => {
            let mut ordering = Ordering::Equal;
            for (i, card) in a.iter().enumerate() {
                match Into::<u8>::into(card.to_owned()).cmp(&Into::<u8>::into(b[i].to_owned())) {
                    Ordering::Equal => continue,
                    x => {
                        ordering = x;
                        break;
                    }
                }
            }
            ordering
        }
        x => x,
    }
}
fn rank_games(games: &mut Games) -> Games {
    games.sort_by(|a, b| compare_hands(&a.0, &b.0));
    games.to_owned()
}

fn parse_input(input: String) -> Games {
    input
        .lines()
        .map(|i| {
            let mut l = i.split(" ");
            (
                l.next()
                    .expect("no hand")
                    .chars()
                    .map(|c| c.into())
                    .collect(),
                l.next()
                    .expect("no bid")
                    .parse::<usize>()
                    .expect("couldn't parse bid"),
            )
        })
        .collect()
}

fn score(hand: &Hand) -> usize {
    if hand.len() != 5 {
        panic!("Invalid hand!");
    }
    //Collect like pairs;
    let mut pairs: HashMap<Card, u8> = HashMap::new();
    for card in hand {
        match pairs.get(&card) {
            Some(amt) => {
                pairs.insert(card.to_owned(), amt + 1);
            }
            None => {
                pairs.insert(card.to_owned(), 1);
            }
        };
    }
    let mut len = pairs.len();
    if pairs.contains_key(&Card::Joker) {
        match len {
            1 => println!("All jokers :D"),
            2 => {
                let jokers = pairs.remove(&Card::Joker).expect("no jokers");
                let other = pairs.clone().into_iter().next().expect("no other cards");
                pairs.insert(other.0, other.1 + jokers);
            },
            3 | 4 => {
                let jokers = pairs.remove(&Card::Joker).expect("no jokers");
                let mut other = pairs.clone().into_iter().collect::<Vec<(Card,u8)>>();
                other.sort_by(|a,b|b.1.cmp(&a.1));
                pairs.insert(other[0].0.clone(), other[0].1 + jokers);
            },
            5 => {
                let jokers = pairs.remove(&Card::Joker).expect("no jokers");
                let other = pairs.clone().into_iter().collect::<Vec<(Card,u8)>>();
                pairs.insert(other[0].0.clone(), other[0].1 + jokers);
            },
            _ => panic!("{pairs:?} invalid!")
        };
    }
    len = pairs.len();
    let combo = match len {
        1 => 7, //Flush!
        2 => {
            match pairs.iter().peekable().peek().unwrap().1 {
                1 | 4 => 6, //Four of a Kind
                2 | 3 => 5, //Full House
                _ => panic!("invalid"),
            }
        }
        3 => {
            let mut combo = 0;
            for (_, amt) in pairs.iter() {
                if *amt == 3 {
                    combo = 4; //Three of a Kind
                };
                if *amt == 2 {
                    combo = 3; //Two Pairs;
                }
            }
            if combo > 0 {
                combo
            } else {
                panic!("invalid")
            }
        }
        4 => 2, //One Pair
        5 => 1, //High Card
        _ => panic!("Invalid pairs"),
    };
    combo
}

type Games = Vec<Game>;
type Game = (Hand, Bid);
type Hand = Vec<Card>;
type Bid = usize;

#[derive(PartialEq, Eq, Hash, Clone, Debug)]
enum Card {
    Ace,
    King,
    Queen,
    Ten,
    Nine,
    Eight,
    Seven,
    Six,
    Five,
    Four,
    Three,
    Two,
    Joker,
}

impl From<char> for Card {
    fn from(value: char) -> Self {
        match value {
            'A' => Self::Ace,
            'K' => Self::King,
            'Q' => Self::Queen,
            'T' => Self::Ten,
            '9' => Self::Nine,
            '8' => Self::Eight,
            '7' => Self::Seven,
            '6' => Self::Six,
            '5' => Self::Five,
            '4' => Self::Four,
            '3' => Self::Three,
            '2' => Self::Two,
            'J' => Self::Joker,
            _ => panic!("invalid card!"),
        }
    }
}

impl Into<u8> for Card {
    fn into(self) -> u8 {
        match self {
            Self::Ace => 13,
            Self::King => 12,
            Self::Queen => 11,
            Self::Ten => 10,
            Self::Nine => 9,
            Self::Eight => 8,
            Self::Seven => 7,
            Self::Six => 6,
            Self::Five => 5,
            Self::Four => 4,
            Self::Three => 3,
            Self::Two => 2,
            Self::Joker => 1,
        }
    }
}
