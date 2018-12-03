use regex::Regex;
use regex::Captures;
use util::Result;

pub fn part1(input: &str) -> Result<usize>  {
    let mut fabric = vec![vec![0; 1000]; 1000];
    input.lines().map(|claim| {
        let claim = parse_line(claim)?;
        Ok(update_fabric(&mut fabric, &claim))
    }).collect::<Result<Vec<_>>>()?;

    Ok(fabric.iter().fold(0, |acc, v| {
        acc + v.iter().filter(|&&x| x > 1).count()
    }))
}

// top left 0,0
#[derive(Debug)]
struct Claim {
    id: usize,
    x: usize,
    y: usize,
    width: usize,
    height: usize,
}

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
            Ok(Claim { id, x, y, width, height})
        })
}

fn update_fabric(fabric: &mut Vec<Vec<i32>>, claim: &Claim) {
    for i in claim.x..claim.x+claim.width {
        for j in claim.y..claim.y+claim.height {
            fabric[i][j] += 1
        }
    }
}


pub fn part2(input: &str) -> Result<i32> {
    Ok(1)
}
