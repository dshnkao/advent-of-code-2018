use util::Result;

fn is_reactive(a: &char, b: &char) -> bool {
    a != b && (a.to_ascii_lowercase() == *b || b.to_ascii_lowercase() == *a)
}

pub fn part1(input: &str) -> Result<usize> {
    let mut reacted_polymer: Vec<char> = Vec::new();
    input.chars()
        .filter(|x| x.is_alphabetic())
        .for_each(|cur| {
            match reacted_polymer.last().cloned() {
                Some(prv) if is_reactive(&prv, &cur) => drop(reacted_polymer.pop()),
                Some(_) => reacted_polymer.push(cur),
                None => reacted_polymer.push(cur),
            }
        });
    Ok(reacted_polymer.len())
}

pub fn part2(input: &str) -> Result<usize> {
    Ok(1)
}
