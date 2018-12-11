use crate::util::Result;

fn is_reactive(a: char, b: char) -> bool {
    a != b && (a.to_ascii_lowercase() == b || b.to_ascii_lowercase() == a)
}

fn react_polymer(polymer: &str, ignore_chars: &[char]) -> Vec<char> {
    let mut reacted_polymer: Vec<char> = Vec::new();
    polymer.chars()
        .filter(|x| x.is_alphabetic())
        .filter(|x| !ignore_chars.iter().any(|c| c == x))
        .for_each(|cur| {
            match reacted_polymer.last().cloned() {
                Some(prv) if is_reactive(prv, cur) => { reacted_polymer.pop(); },
                Some(_) => reacted_polymer.push(cur),
                None => reacted_polymer.push(cur),
            }
        });
    reacted_polymer
}

pub fn part1(input: &str) -> Result<usize> {
    Ok(react_polymer(&input, &[]).len())
}

pub fn part2(input: &str) -> Result<usize> {
    let polymer = (b'a' ..= b'z')
        .map(|c| c as char)
        .map(|c| react_polymer(&input, &[c, c.to_ascii_uppercase()]))
        .min_by_key(|polymer| polymer.len())
        .ok_or("no polymer found")?;

    Ok(polymer.len())
}
