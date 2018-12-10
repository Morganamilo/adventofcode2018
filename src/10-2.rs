use std::io::{self, BufRead};

#[derive(Copy, Clone, Debug)]
struct Point {
    x: i32,
    y: i32,
    dx: i32,
    dy: i32,
}

impl Point {
    fn new(s: &str) -> Point {
        let mut coord = s
                .split(|c: char| !(c.is_ascii_digit() || c == '-'))
                .filter(|x| !x.is_empty())
                .map(|x| x.parse().unwrap());

        Point {
            x: coord.next().unwrap(),
            y: coord.next().unwrap(),
            dx: coord.next().unwrap(),
            dy: coord.next().unwrap(),
        }
    }
}

fn main() {
    let stdin = io::stdin();
    let points = stdin.lock().lines().map(|x| Point::new(&x.unwrap())).collect::<Vec<_>>();

    let max = points.iter().max_by_key(|p| p.y).unwrap().clone();
    let min = points.iter().min_by_key(|p| p.y).unwrap().clone();
    let dist = max.y - min.y;
    let delta = (max.dy).abs() + (min.dy).abs();

    println!("{}", dist / delta);

}
