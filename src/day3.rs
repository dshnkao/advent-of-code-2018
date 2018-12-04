use regex::Captures;
use regex::Regex;
use std::collections::HashMap;
use util::Result;

// top left 0,0
#[derive(Debug)]
struct Claim {
    id: usize,
    coord: Coord,
    width: usize,
    height: usize,
}

#[derive(Debug, PartialEq, Eq, Hash)]
struct Coord {
    x: usize,
    y: usize,
}

type Fabric = HashMap<Coord, usize>;

fn parse_line(claim: &str) -> Result<Claim> {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"#(\d+) @ (\d+),(\d+): (\d+)x(\d+)")
            .expect("invalid regex");
    }

    let parse_num = |cap: &Captures, index: usize, err: &'static str| {
        cap.get(index)
            .ok_or(err)
            .and_then(|x| x.as_str().parse::<usize>().map_err(|_| err))
    };

    RE.captures_iter(claim)
        .next()
        .ok_or("no claim found")
        .and_then(|cap| {
            let id = parse_num(&cap, 1, "claim id not found")?;
            let x = parse_num(&cap, 2, "x not found")?;
            let y = parse_num(&cap, 3, "y not found")?;
            let width = parse_num(&cap, 4, "width not found")?;
            let height = parse_num(&cap, 5, "height not found")?;
            let coord = Coord { x, y };
            Ok(Claim { id, coord, width, height})
        })
}

fn update_fabric(fabric: &mut Fabric, claim: &Claim) {
    for x in claim.coord.x .. claim.coord.x + claim.width {
        for y in claim.coord.y .. claim.coord.y + claim.height {
            *fabric.entry(Coord { x, y } ).or_default() += 1
        }
    }
}

fn non_overlap_claim(fabric: &Fabric, claim: &Claim) -> bool {
    for x in claim.coord.x .. claim.coord.x + claim.width {
        for y in claim.coord.y .. claim.coord.y + claim.height {
            let coord = Coord {x, y};
            if fabric.get(&coord) != Some(&1) {
                return false
            }
        }
    }
    true
}

pub fn part1(input: &str) -> Result<usize> {
    let mut fabric = Fabric::new();
    input.lines().map(|claim| {
        let claim = parse_line(claim)?;
        Ok(update_fabric(&mut fabric, &claim))
    }).collect::<Result<Vec<_>>>()?;

    Ok(fabric.values().filter(|&&x| x > 1).count())
}

pub fn part2(input: &str) -> Result<usize> {

    let mut fabric = Fabric::new();
    let mut claims = Vec::new();

    input.lines().map(|claim| {
        let claim = parse_line(claim)?;
        update_fabric(&mut fabric, &claim);
        claims.push(claim);
        Ok(())
    }).collect::<Result<Vec<_>>>()?;

    claims.into_iter()
        .find(|c| non_overlap_claim(&fabric, &c))
        .ok_or("no claim found")
        .map(|c| c.id)
}
