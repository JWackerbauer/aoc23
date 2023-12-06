pub fn ways_to_win(race_ms: usize, distance: usize) -> usize {
    let possible_races = plot_races(race_ms);
    possible_races.iter().filter(|r| **r > distance).count()
}

pub fn plot_races(race_ms: usize) -> Vec<usize> {
    let mut out = vec![];
    for i in 1..race_ms {
        out.push(calculate_distance(i, race_ms));
    }
    out
}

pub fn calculate_distance(button_ms: usize, race_ms: usize) -> usize {
    if button_ms > race_ms {
        panic!("can't press longer than race");
    }
    let speed = button_ms;
    let travel_time = race_ms - button_ms;
    travel_time * speed
}

#[test]
fn test_calc() {
    assert_eq!(calculate_distance(0, 7), 0);
    assert_eq!(calculate_distance(1, 7), 6);
    assert_eq!(calculate_distance(2, 7), 10);
    assert_eq!(calculate_distance(3, 7), 12);
    assert_eq!(calculate_distance(4, 7), 12);
    assert_eq!(calculate_distance(5, 7), 10);
    assert_eq!(calculate_distance(6, 7), 6);
    assert_eq!(calculate_distance(7, 7), 0);
}