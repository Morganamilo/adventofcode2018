use std::io::{self, BufRead};

#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq)]
struct NanoBot {
    pos: Point,
    rad: i32,
}

impl NanoBot {
    fn new(s: &str) -> NanoBot {
        let mut it = s
            .split(|c: char| !c.is_ascii_digit() && c != '-')
            .filter(|x| !x.is_empty())
            .map(|x| x.parse::<i32>().unwrap());

        NanoBot {
            pos: Point {
                x: it.next().unwrap(),
                y: it.next().unwrap(),
                z:  it.next().unwrap(),
            },
            rad: it.next().unwrap(),
        }
    }
}

#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq)]
struct Point {
    x: i32,
    y: i32,
    z: i32,
}

fn bots_in_range(dist: i32, pairs: &[(i32, i32)]) -> usize {
    pairs.iter()
    .filter(|pair| dist >= pair.0 && dist <= pair.1)
    .count()
}

fn main() {
    let stdin = io::stdin();
    let bots = stdin.lock()
        .lines()
        .map(|x| (NanoBot::new(&x.unwrap())))
        .collect::<Vec<_>>();

    let mut pairs = Vec::new();
    let mut dists = Vec::new();

    for b in &bots {
        let min = b.pos.x.abs() + b.pos.y.abs() + b.pos.z.abs() - b.rad;
        let max = b.pos.x.abs() + b.pos.y.abs() + b.pos.z.abs() + b.rad;

        pairs.push((min, max));
        dists.push(min);
        dists.push(max);
    }
    dists.sort();

    let min = dists
        .iter()
        .rev()
        .max_by_key(|&&dist| bots_in_range(dist, &pairs))
        .unwrap();

    println!("{}", min);
}
