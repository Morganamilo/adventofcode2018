use std::io::{self, BufRead};

#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn new(s: &str) -> Point {
        let mut coord = s
                .split(|c: char| !c.is_ascii_digit())
                .filter(|x| !x.is_empty())
                .map(|x| x.parse().unwrap());

        Point {
            x: coord.next().unwrap(),
            y: coord.next().unwrap(),
        }
    }

    fn distance(self, other: Self) -> i32 {
        (self.x - other.x).abs() + (self.y - other.y).abs()
    }
}

fn main() {
    let stdin = io::stdin();
    let coords = stdin.lock().lines().map(|x| Point::new(&x.unwrap())).collect::<Vec<_>>();
    let mut total = 0;

    let start = Point{
        x: coords.iter().map(|p| p.x).min().unwrap(),
        y: coords.iter().map(|p| p.y).min().unwrap(),

    };

    let end = Point{
        x: coords.iter().map(|p| p.x).max().unwrap(),
        y: coords.iter().map(|p| p.y).max().unwrap(),
    };

    for y in start.y..=end.y {
        for x in start.x..=end.x {
            let point = Point{x, y};
            let dist: i32 = coords.iter().map(|&c| point.distance(c)).sum();
            if dist < 10000 {
                total += 1;
            }
        }
    }

    println!("{}", total);
}
