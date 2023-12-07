use crate::*;

#[test]
fn test_input() {
    let input = r#"32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483"#;
    println!("{:?}", rank_games(&mut parse_input(input.to_string())));
    //assert_eq!(rank_games(&mut parse_input(input)), vec![])
    assert_eq!(get_winnings(input.to_string()), 6440);
}

#[test]
fn test_score() {
    //32T3K
    let one_pair = vec![Card::Three, Card::Two, Card::Ten, Card::Three, Card::King];
    assert_eq!(score(&one_pair), 2);

    //KK677
    let two_pair_1 = vec![Card::King, Card::King, Card::Six, Card::Seven, Card::Seven];
    let score_two_pair_1 = score(&two_pair_1);
    assert_eq!(score_two_pair_1, 3);
    //KTJJT
    let two_pair_2 = vec![Card::King, Card::Ten, Card::Jack, Card::Jack, Card::Ten];
    let score_two_pair_2 = score(&two_pair_2);
    assert_eq!(score_two_pair_2, 3);
    assert_eq!(compare_hands(&two_pair_1, &two_pair_2), Ordering::Greater);

    //T55J5
    let three_1 = vec![Card::Ten, Card::Five, Card::Five, Card::Jack, Card::Five];
    let score_three_1 = score(&three_1);
    assert_eq!(score_three_1, 4);
    //QQQJA
    let three_2 = vec![Card::Queen, Card::Queen, Card::Queen, Card::Jack, Card::Ace];
    let score_three_2 = score(&three_2);
    assert_eq!(score_three_2, 4);
    assert_eq!(compare_hands(&three_1, &three_2), Ordering::Less);
}
