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
    let mut points = stdin.lock().lines().map(|x| Point::new(&x.unwrap())).collect::<Vec<_>>();

    let max = points.iter().max_by_key(|p| p.y).unwrap().clone();
    let min = points.iter().min_by_key(|p| p.y).unwrap().clone();
    let dist = max.y - min.y;
    let delta = (max.dy).abs() + (min.dy).abs();

    for point in &mut points {
        point.x += point.dx * (dist / delta);
        point.y += point.dy * (dist / delta);
    }

    let starty = points.iter().map(|p| p.y).min().unwrap();
    let endy = points.iter().map(|p| p.y).max().unwrap();
    let endx = points.iter().map(|p| p.x).max().unwrap();
    let startx = points.iter().map(|p| p.x).min().unwrap();

    for y in starty..=endy {
        for x in startx..=endx {
            if points.iter().any(|p| p.x == x && p.y == y) {
                print!("#");
            } else {
                print!(" ");
            }
        }

        println!();
    }

}
