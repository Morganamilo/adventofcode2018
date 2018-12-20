use std::io::{self, Read};
use std::collections::HashMap;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum Tile {
    Door,
    Room,
}

#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

fn step(map: &mut HashMap<Point, Tile>, point: &mut Point, c: char) {
     match c {
            'N' => {
                point.y -= 1;
                map.insert(*point, Tile::Door);
                point.y -= 1;
                map.insert(*point, Tile::Room);
            },
            'E' => {
                point.x += 1;
                map.insert(*point, Tile::Door);
                point.x += 1;
                map.insert(*point, Tile::Room);
            },
            'S' => {
                point.y += 1;
                map.insert(*point, Tile::Door);
                point.y += 1;
                map.insert(*point, Tile::Room);
            },
            'W' => {
                point.x -= 1;
                map.insert(*point, Tile::Door);
                point.x -= 1;
                map.insert(*point, Tile::Room);
            },
            _ => panic!("unkown char {}", c),
     }
}

fn parse<'a>(map: &mut HashMap<Point, Tile>, mut point: Point, chars: &[char], mut i: usize) -> usize {
    let original = point;
    while i < chars.len() {
        let c = chars[i];
        if c.is_ascii_alphabetic() {
            step(map, &mut point, c);
            i += 1;
        } else if c == '|' {
            point = original;
            i += 1;
        } else if c == '(' {
            i = parse(map, point, chars, i+1);
        } else if c == ')' {
            return i+1;
        } else {
            i += 1;
        }
    }
    return i;
}

fn dist_map(map: &HashMap<Point, Tile>, seen: &mut HashMap<Point, i32>, point: Point, dist: i32) {
    if seen.get(&point).is_some() {
        return;
    }

    seen.insert(point, dist);

    for (x, y) in &[(0, 1), (0, -1), (1, 0), (-1, 0)] {
        let p = Point { x: point.x + x, y: point.y - y };
        if let Some(&tile) = map.get(&p) {
            if tile == Tile::Door {
                let p = Point { x: point.x + x*2, y: point.y - y*2 };
                dist_map(map, seen, p, dist + 1);
            }
        }
    }

}

fn main() {
    let mut input = String::new();
    let mut map = HashMap::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let input = input.trim().trim_start_matches('^').trim_end_matches('$');
    let chars = input.chars().collect::<Vec<_>>();
    let point = Point { x: 0, y: 0};
    let mut dmap = HashMap::new();

    map.insert(point, Tile::Room);
    parse(&mut map, point, &chars, 0);
    dist_map(&map, &mut dmap, point, 0);
    println!("{}", dmap.values().filter(|&&x| x >= 1000).count());
}
