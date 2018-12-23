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

impl Point {
    fn distance(self, other: Self) -> i32 {
        (self.x - other.x).abs() + (self.y - other.y).abs() + (self.z - other.z).abs()
    }
}

fn main() {
    let stdin = io::stdin();
    let bots = stdin.lock()
        .lines()
        .map(|x| (NanoBot::new(&x.unwrap())))
        .collect::<Vec<_>>();

    let strongest = bots.iter().max_by_key(|x| x.rad).unwrap();
    let mut count = 0;

    for bot in bots.iter() {
        if strongest.pos.distance(bot.pos) <= strongest.rad {
            count += 1;
        }
    }

    println!("{}", count);
}
