mod day1;

use std::env;
use std::io;
use std::io::prelude::*;

fn main() {

    let challenge: String = env::args()
        .nth(1)
        .expect("challenge expected");

    let stdin = io::stdin();
    let lines = stdin.lock()
        .lines()
        .map(|x| x.unwrap());

    let output = match challenge.as_ref() {
        "1a" => day1::part1(lines).to_string(),
        "1b" => day1::part2(lines).to_string(),
        _ => panic!("challenge invalid")
    };
    println!("{}", output);
}
