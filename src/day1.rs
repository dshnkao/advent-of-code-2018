use std::collections::HashSet;
use crate::util::Result;

pub fn part1(input: &str) -> Result<i32> {
    input.lines()
        .map(|x| x.parse::<i32>())
        .map(|x| x.map_err(|_| "integer expected"))
        .sum()
}

pub fn part2(input: &str) -> Result<i32> {
    let mut frequency = 0;
    let mut freq_history = HashSet::new();
    freq_history.insert(frequency);

    let frequencies = input
        .lines()
        .map(|x| x.parse::<i32>().map_err(|_| "integer expected"))
        .collect::<Result<Vec<_>>>()?
        .into_iter()
        .cycle();

    for i in frequencies {
        frequency += i;
        if freq_history.contains(&frequency) { break; }
        freq_history.insert(frequency);
    }
    Ok(frequency)
}
