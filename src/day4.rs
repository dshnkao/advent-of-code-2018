use chrono::{Duration, NaiveDateTime, Timelike};
use regex::Regex;
use std::collections::HashMap;
use std::str::FromStr;
use util::Result;

#[derive(Debug)]
struct Entry {
    time: NaiveDateTime,
    guard_id: usize,
    action: Action,
}

#[derive(Debug)]
enum Action {
    FallsAsleep,
    WakesUp,
    BeginsShift,
}

impl FromStr for Action {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self> {
        match s {
            "falls asleep" => Ok(Action::FallsAsleep),
            "wakes up" => Ok(Action::WakesUp),
            "begins shift" => Ok(Action::BeginsShift),
            _ => Err("failed to parse action")
        }
    }
}

struct Metrics {
    total_sleep_time: usize,
    slept_per_minute: [usize; 60],
}

struct Minute {
    minute: usize,
    count: usize,
}

impl Metrics {
    fn update(&mut self, sleep_at: &NaiveDateTime, wakeup_at: &NaiveDateTime) {
        let mut cursor = sleep_at.clone();
        while cursor < *wakeup_at {
            self.total_sleep_time += 1;
            self.slept_per_minute[cursor.minute() as usize] += 1;
            cursor += Duration::minutes(1)
        }
    }
    fn most_slept_minute(&self) -> Result<Minute> {
        self.slept_per_minute.into_iter()
            .enumerate()
            .max_by_key(|(_, &x)| x)
            .map(|(minute, &count)| Minute { minute, count })
            .ok_or("didn't sleep")
    }
}

type GuardMetrics = HashMap<usize, Metrics>;

fn parse_line(entry: &str, curr_guard_id: &mut Option<usize>) -> Result<Entry> {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"\[(\d{4}-\d{2}-\d{2} \d{2}:\d{2})\] (.*)")
            .expect("invalid regex");

        static ref RE_BEGIN_SHIFT: Regex = Regex::new(r"Guard #(\d+) begins shift")
            .expect("invalid regex");
    }

    let mut parse_action = |guard_action: &str, time: NaiveDateTime| {
        RE_BEGIN_SHIFT.captures_iter(guard_action)
            .next()
            .map(|cap| {
                let guard_id = cap.get(1).and_then(|x| x.as_str().parse::<usize>().ok())?;
                let action = Action::BeginsShift;
                *curr_guard_id = Some(guard_id);
                Some(Entry { time, guard_id, action } )
            })
            .unwrap_or({
                let guard_id = curr_guard_id.clone()?;
                guard_action.parse::<Action>()
                    .ok()
                    .map(|action| Entry { time, guard_id, action })
            })
    };

    RE.captures_iter(entry)
        .next()
        .ok_or("invalid entry")
        .and_then(|cap| {
            let time = cap.get(1)
                .and_then(|x| NaiveDateTime::parse_from_str(x.as_str(), "%Y-%m-%d %H:%M").ok())
                .ok_or("failed to parse timestamp")?;
            let entry = cap.get(2)
                .and_then(|x| parse_action(x.as_str(), time))
                .ok_or("failed to parse action");
            entry
        })
}

fn read_schedule(input: &str) -> Result<GuardMetrics> {
    let mut timeline: Vec<&str> = input.lines().collect();
    timeline.sort();

    let mut guard_metrics = GuardMetrics::new();
    let mut curr_guard_id: Option<usize> = None;
    let mut slept_at: Option<NaiveDateTime> = None;

    let entries = timeline.iter()
        .map(|x| parse_line(x, &mut curr_guard_id))
        .collect::<Result<Vec<_>>>()?;

    entries.iter().map(|x| {
        match x.action {
            Action::BeginsShift => Ok(()),
            Action::FallsAsleep => {
                slept_at = Some(x.time);
                Ok(())
            },
            Action::WakesUp => {
                let last_slept_at = slept_at.ok_or("guard woke up without falling asleep")?;
                let metrics = guard_metrics.entry(x.guard_id)
                    .or_insert(Metrics { total_sleep_time: 0, slept_per_minute: [0; 60] });
                metrics.update(&last_slept_at, &x.time);
                slept_at = None;
                Ok(())
            },
        }
    }).collect::<Result<Vec<_>>>()?;

    Ok(guard_metrics)
}

pub fn part1(input: &str) -> Result<usize> {
    let guard_metrics = read_schedule(input)?;
    let (guard_id, metrics) = guard_metrics.iter()
        .max_by_key(|(_, v)| v.total_sleep_time)
        .ok_or("most slept guard not found")?;
    let most_slept_minute = metrics.most_slept_minute()?;
    Ok(guard_id * most_slept_minute.minute)
}

pub fn part2(input: &str) -> Result<usize> {
    let guard_metrcis = read_schedule(input)?;
    let (guard_id, metrics) = guard_metrcis.iter()
        .max_by_key(|(_, v)| v.most_slept_minute().map(|x| x.count).unwrap_or(0))
        .ok_or("no guard found")?;
    let most_slept_minute = metrics.most_slept_minute()?;
    Ok(guard_id * most_slept_minute.minute)
}
