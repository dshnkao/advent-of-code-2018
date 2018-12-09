use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::collections::HashSet;
use util::Result;
use regex::{Captures, Regex};

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
struct Step {
    value: char
}

impl PartialOrd for Step {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(&other))
    }
}

impl Ord for Step {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.value > other.value { Ordering::Less }
        else if self.value < other.value { Ordering::Greater }
        else { Ordering::Equal }
    }
}

struct Path {
    subsequents: HashMap<Step, HashSet<Step>>,
    prerequisites: HashMap<Step, HashSet<Step>>,
}

impl Path {
    fn new() -> Path {
        Path {
            subsequents: HashMap::new(),
            prerequisites: HashMap::new(),
        }
    }

    fn add(&mut self, step: Step, next: Step) {
        self.subsequents.entry(step).or_default().insert(next);
        self.prerequisites.entry(step).or_default();
        self.prerequisites.entry(next).or_default().insert(step);
    }

    fn find_path(&mut self) -> Vec<Step> {
        let unlocked: Vec<Step> = self.prerequisites.iter()
            .filter(|(_, v)| v.is_empty())
            .map(|(&step, _)| step)
            .collect();
        let mut queue = unlocked.iter()
            .fold(BinaryHeap::new(), |mut acc, &step| {
                self.prerequisites.entry(step).or_default().remove(&step);
                acc.push(step);
                acc
            });
        let mut path = Vec::new();
        while let Some(current_step) = queue.pop() {
            path.push(current_step);
            let prerequisites = &mut self.prerequisites;
            self.subsequents.get(&current_step).iter()
                .flat_map(|blocked_steps| blocked_steps.iter())
                .for_each(|&blocked| {
                    let prereq = prerequisites.entry(blocked).or_default();
                    prereq.remove(&current_step);
                    if prereq.is_empty() {
                        queue.push(blocked)
                    }
                });
        }
        path
    }
}

fn parse_line(line: &str) -> Result<(Step, Step)> {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"^Step (\w) must be finished before step (\w) can begin.$")
            .expect("invalid regex");
    }
    let parse_char = |cap: &Captures, index: usize, err: &'static str| {
        cap.get(index)
            .and_then(|x| x.as_str().chars().next())
            .ok_or(err)
    };
    RE.captures_iter(line)
        .next()
        .ok_or("invalid line")
        .and_then(|cap| {
            let s1 = parse_char(&cap, 1, "failed to parse first char")?;
            let s2 = parse_char(&cap, 2, "failed to parse second char")?;
            Ok((Step { value: s1 }, Step { value: s2 }))
        })
}

pub fn part1(input: &str) -> Result<String> {
    let mut path = input.lines()
        .filter_map(|x| parse_line(x).ok())
        .fold(Path::new(), |mut path, (s1, s2)| {
            path.add(s1, s2);
            path
        });
    Ok(path.find_path().into_iter().map(|x| x.value).collect())
}


pub fn part2(input: &str) -> Result<usize> {
    Err("todo")
}
