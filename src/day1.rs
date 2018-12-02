use std::collections::HashSet;

pub fn part1(input: &str) -> i32 {
    input.lines()
        .map(|x| x.parse::<i32>())
        .map(|x| x.expect("integer expected"))
        .sum()
}

pub fn part2(input: &str) -> i32 {
    let mut frequency = 0;
    let mut freq_history = HashSet::new();
    freq_history.insert(frequency.clone());

    let frequencies = input
        .lines()
        .map(|x| x.parse::<i32>().expect("integer expected"))
        .collect::<Vec<i32>>()
        .into_iter()
        .cycle();

    for i in frequencies {
        frequency += i;
        if freq_history.contains(&frequency) { break; }
        freq_history.insert(frequency);
    }
    frequency
}
