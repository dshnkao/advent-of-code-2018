
use std::collections::HashMap;
use std::cmp::{max, min, Ordering};
use crate::util::Result;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
struct Coord {
    x: usize,
    y: usize,
}

fn manhattan_distance(a: &Coord, b: &Coord) -> usize {
    max(a.x, b.x) - min(a.x, b.x) + max(a.y, b.y) - min(a.y, b.y)
}

fn parse_line(line: &str) -> Result<Coord> {
    let matches: Vec<_> = line.split(", ").collect();
    let x = matches.get(0)
        .and_then(|x| x.parse::<usize>().ok())
        .ok_or("x not found")?;
    let y = matches.get(1)
        .and_then(|x| x.parse::<usize>().ok())
        .ok_or("y not found")?;
    Ok(Coord { x, y })
}

#[derive(Debug)]
struct Grid {
    min_x: usize,
    min_y: usize,
    max_x: usize,
    max_y: usize,
    buf: HashMap<Coord, Option<usize>>
}

impl Grid {
    fn new() -> Grid {
        Grid {
            min_x: std::usize::MAX, min_y: std::usize::MAX,
            max_x: 0, max_y: 0,
            buf: HashMap::new(),
        }
    }

    fn insert(&mut self, coord: Coord) {
        self.min_x = min(self.min_x, coord.x);
        self.min_y = min(self.min_y, coord.y);
        self.max_x = max(self.max_x, coord.x);
        self.max_y = max(self.max_y, coord.y);
        self.buf.insert(coord, Some(0));
    }

    fn on_boundary(&self, coord: &Coord) -> bool {
        coord.x == self.min_x || coord.x == self.max_x ||
            coord.y == self.min_y || coord.y == self.max_y
    }

    fn increment_closest(&mut self, location: Coord) {
        let find_min = |acc: Option<(Coord, usize, bool)>, (c2, dist2): (Coord, usize)| {
            acc.map(|(c1, dist1, unique)| {
                match dist1.cmp(&dist2) {
                    Ordering::Less => Some((c1, dist1, unique)),
                    Ordering::Greater => Some((c2, dist2, true)),
                    Ordering::Equal => Some((c1, dist1, false)),
                }
            }).unwrap_or_else(|| Some((c2, dist2, true)))
        };
        let min_coord = self.buf.keys()
            .map(|&coord| (coord, manhattan_distance(&coord, &location)))
            .fold(None, find_min);

        match min_coord {
            Some((coord, _, _)) if self.on_boundary(&location) => {
                *self.buf.entry(coord).or_default() = None;
            },
            Some((coord, _, true)) => {
                let counter = self.buf.entry(coord).or_default();
                *counter = counter.map(|x| x + 1);
            },
            _ => (),
        }
    }

    fn distance_to_all_coord(&self, location: &Coord) -> usize {
        self.buf.keys()
            .map(|&coord| manhattan_distance(&coord, &location))
            .sum()
    }
}

pub fn part1(input: &str) -> Result<usize> {
    let mut grid = input.lines()
        .map(|x| parse_line(x))
        .filter_map(::std::result::Result::ok)
        .fold(Grid::new(), |mut grid, x| {
            grid.insert(x);
            grid
        });

    for x in grid.min_x ..= grid.max_x {
        for y in grid.min_y ..= grid.max_y {
            grid.increment_closest(Coord { x, y })
        }
    }

    grid.buf.into_iter()
        .max_by_key(|&(_, s)| s)
        .and_then(|(_, size)| size)
        .ok_or("no closest coord found")
}


pub fn part2(input: &str) -> Result<usize> {
    let grid = input.lines()
        .map(|x| parse_line(x))
        .filter_map(::std::result::Result::ok)
        .fold(Grid::new(), |mut grid, x| {
            grid.insert(x);
            grid
        });

    let mut region_size = 0;
    for x in grid.min_x ..= grid.max_x {
        for y in grid.min_y ..= grid.max_y {
            let size = grid.distance_to_all_coord(&Coord { x, y } );
            if size < 10000 {
                region_size += 1;
            }
        }
    }

    Ok(region_size)
}

