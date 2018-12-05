#[macro_use] extern crate lazy_static;
extern crate chrono;
extern crate regex;

mod day1;
mod day2;
mod day3;
mod day4;
mod util;

use std::env;
use std::io::BufReader;
use std::io::prelude::*;
use std::fs::File;
use std::fmt::Display;
use util::Result;

fn main() -> Result<()> {

    let challenge: String = env::args()
        .nth(1)
        .ok_or("challenge expected")?;

    let result = match challenge.as_ref() {
        "1a" => Puzzle { solution: day1::part1, input: "01.txt" }.solve(),
        "1b" => Puzzle { solution: day1::part2, input: "01.txt" }.solve(),
        "2a" => Puzzle { solution: day2::part1, input: "02.txt" }.solve(),
        "2b" => Puzzle { solution: day2::part2, input: "02.txt" }.solve(),
        "3a" => Puzzle { solution: day3::part1, input: "03.txt" }.solve(),
        "3b" => Puzzle { solution: day3::part2, input: "03.txt" }.solve(),
        "4a" => Puzzle { solution: day4::part1, input: "04.txt" }.solve(),
        "4b" => Puzzle { solution: day4::part2, input: "04.txt" }.solve(),
        _ => Err("invalid challenge"),
    }?;

    println!("{}", result);
    Ok(())
}

struct Puzzle<F, A: Display>
where
    F: Fn(&str) -> Result<A>
{
    solution: F,
    input: &'static str,
}

impl <F, A: Display> Puzzle<F, A>
where
    F: Fn(&str) -> Result<A>
{
    fn solve(&self) -> Result<String> {
        let mut buf = String::new();
        let file_location = "./data/".to_owned() + self.input;
        let f = File::open(file_location).map_err(|_| "input file not found under ./data/*.txt")?;
        BufReader::new(f).read_to_string(&mut buf).map_err(|_| "unable to read file")?;
        (self.solution)(buf.as_ref()).map(|x| x.to_string())
    }
}
