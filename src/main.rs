mod day1;

use std::env;
use std::io::BufReader;
use std::io::prelude::*;
use std::fs::File;
use std::fmt::Display;

fn main() {

    let challenge: String = env::args()
        .nth(1)
        .expect("challenge expected");

    match challenge.as_ref() {
        "1a" => Puzzle { solution: day1::part1, input: "01.txt" }.solve(),
        "1b" => Puzzle { solution: day1::part2, input: "01.txt" }.solve(),
        _ => panic!("challenge invalid")
    };
}

struct Puzzle<F, A: Display>
where
    F: Fn(&str) -> A
{
    solution: F,
    input: &'static str,
}

impl <F, A: Display> Puzzle<F, A>
where
    F: Fn(&str) -> A
{
    fn solve(&self) {
        let mut buf = String::new();
        let file_location = "./data/".to_owned() + self.input;
        let f = File::open(file_location).expect("input file not found under ./data/*.txt");
        BufReader::new(f).read_to_string(&mut buf).expect("unable to read file");
        let result = (self.solution)(buf.as_ref());
        println!("{}", result);
    }
}
