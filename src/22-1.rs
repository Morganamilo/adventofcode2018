use std::io::{self, BufRead};
use std::collections::HashMap;

#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq)]
struct Point {
    x: u32,
    y: u32,
}

impl Point {
    fn new(s: &str) -> Point {
        let mut it = s
            .split(|c: char| !c.is_ascii_digit())
            .filter(|x| !x.is_empty())
            .map(|x| x.parse::<u32>().unwrap());

        Point {
            x: it.next().unwrap(),
            y: it.next().unwrap(),
        }
    }

    fn left(&self) -> Point {
        Point{ x: self.x - 1, y: self.y }
    }

    fn up(&self) -> Point {
        Point{ x: self.x, y: self.y - 1}
    }
}

fn risk(point: Point, target: Point, depth: u32, geologic_map: &mut HashMap<Point, u32>) -> u32 {
    erosion_level(point, target, depth, geologic_map) % 3
}

fn erosion_level(point: Point, target: Point, depth: u32, geologic_map: &mut HashMap<Point, u32>) -> u32 {
    (geologic_index(point, target, depth, geologic_map) + depth) % 20183
}

fn geologic_index(point: Point, target: Point, depth: u32, geologic_map: &mut HashMap<Point, u32>) -> u32 {

    if let Some(&index) = geologic_map.get(&point) {
        return index;
    }

    let index = if point.x == 0 && point.y == 0 {
        0
    } else if point.x == target.x && point.y == target.y {
        0
    } else if point.y == 0 {
        point.x * 16807
    } else if point.x == 0 {
        point.y * 48271
    } else {
        erosion_level(point.left(), target, depth, geologic_map) *
            erosion_level(point.up(), target, depth, geologic_map)
    };

    geologic_map.insert(point, index);
    index
}

fn main() {
    let stdin = io::stdin();
    let lines = stdin.lock().lines().map(|x| x.unwrap()).collect::<Vec<_>>();
    let depth = lines[0]
        .split(|c: char| !c.is_ascii_digit())
        .filter(|x| !x.is_empty())
        .map(|x| x.parse::<u32>().unwrap())
        .next()
        .unwrap();
    let target = Point::new(&lines[1]);

    let mut total = 0;
    let mut geologic_map = HashMap::new();

    for x in 0..= target.x {
        for y in 0..=target.y {
            let point = Point { x, y };
            total += risk(point, target, depth, &mut geologic_map);
        }
    }

    println!("{}", total);
}
