use std::io::{self, BufRead};

#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq)]
struct Point {
    x: i32,
    y: i32,
    z: i32,
    t: i32,
}

impl Point {
    fn new(s: &str) -> Point {
        let mut it = s
            .split(|c: char| !c.is_ascii_digit() && c != '-')
            .filter(|x| !x.is_empty())
            .map(|x| x.parse::<i32>().unwrap());

        Point {
            x: it.next().unwrap(),
            y: it.next().unwrap(),
            z: it.next().unwrap(),
            t: it.next().unwrap(),
        }
    }


    fn distance(self, other: Self) -> i32 {
        (self.x - other.x).abs() +
            (self.y - other.y).abs() +
            (self.z - other.z).abs() +
            (self.t - other.t).abs()
    }
}

fn add(constalations: &mut Vec<Vec<Point>>, point: Point) {
    for constalation in constalations.iter_mut() {
        if constalation.iter().find(|&&p| point.distance(p) <= 3 ).is_some() {
            constalation.push(point);
            return;
        }
    }

    println!("new {:?}", point);
    constalations.push(vec![point]);
}

fn can_merge(c1: &[Point], c2: &[Point]) -> bool {
    for p1 in c1 {
        for p2 in c2 {
            if p1.distance(*p2) <= 3 {
                return true;
            }
        }
    }

    false
}

fn merge(constalations: &mut Vec<Vec<Point>>) -> bool {
    for c1 in 0..constalations.len() {
        for c2 in c1+1..constalations.len() {
            if can_merge(&constalations[c1], &constalations[c2]) {
                let to_merge = &mut constalations[c2].clone();
                constalations[c1].append(to_merge);
                constalations[c2].clear();
                return true;
            }
        }
    }

    false
}

fn main() {
    let mut constalations = vec![];

    let stdin = io::stdin();
    for point in stdin.lock().lines().map(|x| (Point::new(&x.unwrap()))) {
        add(&mut constalations, point);
    }

    while merge(&mut constalations) {}
    constalations.retain(|x| !x.is_empty());

    println!("{}", constalations.len());
}
