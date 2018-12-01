use std::collections::HashSet;

pub fn part1(iter: impl Iterator<Item = String>) -> i32 {
    iter.map(|x| x.parse::<i32>().unwrap())
        .sum()
}

pub fn part2(iter: impl Iterator<Item = String>) -> i32 {
    let mut frequency = 0;
    let mut freq_history = HashSet::new();
    freq_history.insert(frequency.clone());

    let frequencies: Vec<i32> = iter
        .map(|x| x.parse::<i32>().unwrap())
        .collect();

    for i in frequencies.into_iter().cycle() {
        frequency += i;
        if freq_history.contains(&frequency) { break; }
        freq_history.insert(frequency);
    }
    frequency
}
