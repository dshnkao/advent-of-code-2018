#[macro_use] extern crate criterion;

extern crate aoc_lib;

use aoc_lib::*;

use criterion::Criterion;

fn day1a(c: &mut Criterion) {
    let buf = include_str!("../data/01.txt");
    c.bench_function("day1a", move |b| b.iter(|| day1::part1(&buf)));
}

fn day1b(c: &mut Criterion) {
    let buf = include_str!("../data/01.txt");
    c.bench_function("day1b", move |b| b.iter(|| day1::part2(&buf)));
}

fn day2a(c: &mut Criterion) {
    let buf = include_str!("../data/02.txt");
    c.bench_function("day2a", move |b| b.iter(|| day2::part1(&buf)));
}

fn day2b(c: &mut Criterion) {
    let buf = include_str!("../data/02.txt");
    c.bench_function("day2b", move |b| b.iter(|| day2::part2(&buf)));
}

fn day3a(c: &mut Criterion) {
    let buf = include_str!("../data/03.txt");
    c.bench_function("day3a", move |b| b.iter(|| day3::part1(&buf)));
}

fn day3b(c: &mut Criterion) {
    let buf = include_str!("../data/03.txt");
    c.bench_function("day3b", move |b| b.iter(|| day3::part2(&buf)));
}

fn day4a(c: &mut Criterion) {
    let buf = include_str!("../data/04.txt");
    c.bench_function("day4a", move |b| b.iter(|| day4::part1(&buf)));
}

fn day4b(c: &mut Criterion) {
    let buf = include_str!("../data/04.txt");
    c.bench_function("day4b", move |b| b.iter(|| day4::part1(&buf)));
}

fn day5a(c: &mut Criterion) {
    let buf = include_str!("../data/05.txt");
    c.bench_function("day5a", move |b| b.iter(|| day5::part1(&buf)));
}

fn day5b(c: &mut Criterion) {
    let buf = include_str!("../data/05.txt");
    c.bench_function("day5b", move |b| b.iter(|| day5::part2(&buf)));
}

fn day6a(c: &mut Criterion) {
    let buf = include_str!("../data/06.txt");
    c.bench_function("day6a", move |b| b.iter(|| day6::part1(&buf)));
}

fn day6b(c: &mut Criterion) {
    let buf = include_str!("../data/06.txt");
    c.bench_function("day6b", move |b| b.iter(|| day6::part2(&buf)));
}

fn day7a(c: &mut Criterion) {
    let buf = include_str!("../data/07.txt");
    c.bench_function("day7a", move |b| b.iter(|| day7::part1(&buf)));
}

fn day7b(c: &mut Criterion) {
    let buf = include_str!("../data/07.txt");
    c.bench_function("day7b", move |b| b.iter(|| day7::part2(&buf)));
}

criterion_group!(
    benches,
    day1a,
    day1b,
    day2a,
    day2b,
    day3a,
    day3b,
    day4a,
    day4b,
    day5a,
    day5b,
    day6a,
    day6b,
    day7a,
    day7b,
);

criterion_main!(benches);
