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
    assert_eq!(get_winnings(input.to_string()), 5905);
}

#[test]
fn test_score() {
    //32T3K
    let one = vec![Card::Three, Card::Two, Card::Ten, Card::Three, Card::King];
    assert_eq!(score(&one), 2);

    //KK677
    let two = vec![Card::King, Card::King, Card::Six, Card::Seven, Card::Seven];
    assert_eq!(score(&two), 3);
    
    //KTJJT
    let four_1 = vec![Card::King, Card::Ten, Card::Joker, Card::Joker, Card::Ten];
    assert_eq!(score(&four_1), 6);
    
    //T55J5
    let four_2 = vec![Card::Ten, Card::Five, Card::Five, Card::Joker, Card::Five];
    assert_eq!(score(&four_2), 6);
    
    //QQQJA
    let four_3 = vec![Card::Queen, Card::Queen, Card::Queen, Card::Joker, Card::Ace];
    assert_eq!(score(&four_3), 6);

    assert_eq!(compare_hands(&one, &two), Ordering::Less);
    assert_eq!(compare_hands(&two, &four_1), Ordering::Less);
    assert_eq!(compare_hands(&four_2, &four_3), Ordering::Less);
    assert_eq!(compare_hands(&four_1, &four_3), Ordering::Greater);   
}
