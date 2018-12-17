use std::io::{self, BufRead};
use std::collections::HashMap;

#[derive(Clone, Debug, Hash, Eq, PartialEq, Ord, PartialOrd)]
struct Scan {
    minx: usize,
    miny: usize,
    maxx: usize,
    maxy: usize,
    map: Vec<Vec<Tile>>,
}

impl Scan {
    fn new(minx: usize, miny: usize, maxx: usize, maxy: usize) -> Scan {
        let map = vec![vec![Tile::Sand; maxy - miny + 1]; maxx - minx + 1 + 2];
        let minx = minx - 1;
        let maxx = maxx + 1;

        Scan {
            minx,
            miny,
            maxx,
            maxy,
            map,
        }
    }

    fn get(&self, point: Point) -> Tile {
        if point.x < self.minx - 1|| point.y < self.miny || point.x > self.maxx || point.y > self.maxy {
            return Tile::Sand;
        }
        let x = point.x - self.minx;
        let y = point.y - self.miny;
        self.map[x][y]
    }

    fn set(&mut self, point: Point, tile: Tile) {
        if point.x < self.minx || point.y < self.miny || point.x > self.maxx || point.y > self.maxy {
            return;
        }
        let x = point.x - self.minx;
        let y = point.y - self.miny;
        self.map[x][y] = tile;
    }

    fn free(&self, point: Point) -> bool {
        let tile =  self.get(point);
        tile == Tile::Sand || tile == Tile::Passed
    }

    fn total(&self) -> (usize, usize) {
        let mut passed = 0;
        let mut settled = 0;

        for outer in self.map.iter() {
            for tile in outer {
                match tile {
                    Tile::Passed => passed += 1,
                    Tile::Settled => settled += 1,
                    _ => (),
                }
            }
        }


        (passed, settled)
    }
}

#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq, Ord, PartialOrd)]
struct Point {
    y: usize,
    x: usize,
}

impl Point {
    fn down(&self) -> Point {
        Point { x: self.x, y: self.y + 1}
    }

    fn left(&self) -> Point {
        Point { x: self.x - 1, y: self.y}
    }

    fn right(&self) -> Point {
        Point { x: self.x + 1, y: self.y}
    }
}

#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq, Ord, PartialOrd)]
enum Tile {
    Clay,
    Settled,
    Passed,
    Sand,
}

fn spread(scan: &mut Scan, mut water: Point) {
    if !scan.free(water) || water.y > scan.maxy {
        return;
    }

    let mut maxl = water.x;
    let mut maxr = water.x;
    let mut closed = true;

    spread(scan, water.down());
    scan.set(water, Tile::Passed);

    if scan.free(water.down()) {
        return;
    }

    while scan.free(water.left()) {
        water.x -= 1;
        maxl = water.x;
        spread(scan, water.down());
        scan.set(water, Tile::Passed);

        if scan.free(water.down()) {
            closed = false;
            break;
        }
    }

    while scan.free(water.right()) {
        water.x += 1;
        maxr = water.x;
        spread(scan, water.down());
        scan.set(water, Tile::Passed);

        if scan.free(water.down()) {
            closed = false;
            break;
        }
    }

    if closed {
        let y = water.y;
        for x in maxl..=maxr {
            scan.set(Point{ x, y }, Tile::Settled);
        }
    }
}

fn main() {
    let stdin = io::stdin();
    let mut map = HashMap::new();

    for line in stdin.lock().lines().map(|x| x.unwrap()) {
        let line = line.split(|c: char| !c.is_alphanumeric())
            .filter(|x| !x.is_empty())
            .collect::<Vec<_>>();

        let point = line[1].parse().unwrap();
        let range = line[3].parse().unwrap()..=line[4].parse().unwrap();
        for pos in range {
            if line[0] == "x" {
                map.insert(Point { x: point, y: pos }, Tile::Clay);
            } else {
                map.insert(Point { x: pos, y: point }, Tile::Clay);
            }
        }
    }

    let maxy = map.keys().map(|p| p.y).max().unwrap();
    let miny = map.keys().map(|p| p.y).min().unwrap();
    let maxx = map.keys().map(|p| p.x).max().unwrap();
    let minx = map.keys().map(|p| p.x).min().unwrap();

    let mut scan = Scan::new(minx, miny, maxx, maxy);

    for (&point, &tile) in map.iter() {
        scan.set(point, tile);
    }

    spread(&mut scan, Point{x: 500, y: miny});

    let (passed, settled) = scan.total();

    println!("passed:  {}", passed);
    println!("settled: {}", settled);
    println!("total:   {}", passed + settled);
}
