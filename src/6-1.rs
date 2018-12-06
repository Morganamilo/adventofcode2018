use std::io::{self, BufRead};

#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq)]
struct Point {
    x: i32,
    y: i32,
    area: i32,
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
            area: 0,
        }
    }

    fn distance(self, other: Self) -> i32 {
        (self.x - other.x).abs() + (self.y - other.y).abs()
    }

    fn on_edge(&self, start: Self, end: Self) -> bool {
        self.x == start.x || self.x == end.x || self.y == start.y || self.y == end.y
    }
}

fn main() {
    let stdin = io::stdin();
    let mut coords = stdin.lock().lines().map(|x| Point::new(&x.unwrap())).collect::<Vec<_>>();

    let start = Point{
        x: coords.iter().map(|p| p.x).min().unwrap(),
        y: coords.iter().map(|p| p.y).min().unwrap(),
        area: 0,
    };

    let end = Point{
        x: coords.iter().map(|p| p.x).max().unwrap(),
        y: coords.iter().map(|p| p.y).max().unwrap(),
        area: 0,
    };

    for x in start.x..=end.x {
        for y in start.y..=end.y {
            let point = Point{x, y, area: 0};

            let min_dist = coords
                .iter()
                .map(|&c| point.distance(c))
                .min()
                .unwrap();

            let mut c = coords
                .iter_mut()
                .filter(|c| point.distance(**c) == min_dist);

            if let Some(coord) = c.next() {
                if c.next().is_none() {
                    if point.on_edge(start, end) {
                        coord.area = -1
                    } else if coord.area != -1 {
                        coord.area += 1;
                    }
                }
            }
        }
    }

    println!("{}", coords.into_iter().map(|p| p.area).max().unwrap());
}
