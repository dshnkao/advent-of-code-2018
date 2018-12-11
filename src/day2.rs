use std::collections::HashMap;
use crate::util::Result;

pub fn part1(input: &str) -> Result<i32> {
    let count = input.lines()
        .map(|x| find_repeated(x))
        .fold(TotalRepeated { twice: 0, thrice: 0 }, |mut acc, x| {
              acc.increment(x);
              acc
        });
    Ok(count.twice * count.thrice)
}

struct TotalRepeated {
    twice: i32,
    thrice: i32,
}

impl TotalRepeated {
    fn increment(&mut self, repeat: Repeated) {
        self.twice += repeat.twice as i32;
        self.thrice += repeat.thrice as i32;
    }
}

struct Repeated {
    twice: bool,
    thrice: bool,
}

fn find_repeated(s: &str) -> Repeated {
    let mut char_count = HashMap::new();
    let mut twice: i32 = 0;
    let mut thrice: i32 = 0;

    for c in s.chars() {
        let counter = char_count.entry(c).or_insert(0);
        *counter += 1;
        match counter {
            2 => twice += 1,
            3 => { twice -= 1; thrice +=1; },
            4 => thrice -= 1,
            _ => ()
        }
    }
    Repeated { twice: twice > 0, thrice: thrice > 0 }
}


pub fn part2(input: &str) -> Result<String> {
    let lines: Vec<&str> = input.lines().collect();
    let mut c = 0;
    loop {
        let removed_char = lines.iter().map(|&x| {
            let mut s = String::from(x);
            s.remove(c);
            s
        }).collect();

        if let Some(s) = find_same_string(removed_char) {
            return Ok(s);
        }

        c += 1;
    }
}

fn find_same_string(mut v: Vec<String>) -> Option<String> {
    v.sort();
    for i in 0..v.len()-1 {
        if v[i] == v[i+1] {
            return Some(v[i].to_owned());
        }
    }
    None
}
