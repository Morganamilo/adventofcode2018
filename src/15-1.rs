use std::io::{self, BufRead};
use std::collections::{HashMap};

#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq, Ord, PartialOrd)]
struct Point {
    y: i32,
    x: i32,
}

#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq, Ord, PartialOrd)]
struct Unit {
    pos: Point,
    hp: i32,
    team: char,
}

impl Unit {
    fn new(pos: Point, team: char) -> Unit {
        let hp = 200;

        Unit{ pos, hp, team }
    }
}

impl Point {
    fn adjacent(self) -> Vec<Point> {
        vec![
            Point{ x: self.x, y: self.y - 1},
            Point{ x: self.x - 1, y: self.y},
            Point{ x: self.x + 1, y: self.y},
            Point{ x: self.x, y: self.y + 1},
        ]
    }
}

fn tile_clear(map: &HashMap<Point, char>, units: &Vec<Unit>, pos: Point) -> bool {
    if *map.get(&pos).unwrap() == '#' {
        return false
    }

    units.iter().find(|u| u.pos == pos).is_none()
}

fn possible_attack_tiles(map: &HashMap<Point, char>,  units: &Vec<Unit>, team: char) -> Vec<Point> {
    units
        .iter()
        .filter(|u| u.team != team)
        .flat_map(|u| u.pos.adjacent())
        .filter(|&p| tile_clear(map, units, p))
        .collect::<Vec<_>>()
}


fn get_dist_map(
    map: &HashMap<Point, char>, units: &Vec<Unit>, from: Point,) -> HashMap<Point, i32> {
    let mut seen = HashMap::new();
    seen.insert(from, 0);

    loop {
        let mut points = seen.iter().map(|(a, b)| (*a, *b)).collect::<Vec<_>>();
        let mut inserted = false;
        let seen2 = seen.clone();
        points.sort_by_key(|&(p, d)| d);

        for (point, d) in points {
            for &point in &point.adjacent() {
                if tile_clear(map, units, point) && seen2.get(&point).is_none() {
                    seen.insert(point, d + 1);
                    inserted = true;
                }
            }
        }

        if !inserted {
            break
        }
    }

    seen
}

fn try_attack(map: &HashMap<Point, char>, units: &mut Vec<Unit>, unit: Unit) -> Option<usize> {
    let mut adjacent = unit.pos.adjacent()
        .into_iter()
        .filter_map(|p| units.iter().cloned().enumerate().find(|(_, u)| u.pos == p))
        .filter(|(i, u)| u.team != unit.team)
        .collect::<Vec<_>>();

    adjacent.sort_by(|a, b| a.1.hp.cmp(&b.1.hp).then(a.1.pos.cmp(&b.1.pos)));

    if let Some((index, _)) = adjacent.first().cloned() {
        return Some(index)
    }

    None
}

fn round(map: &HashMap<Point, char>, units: &mut Vec<Unit>) -> bool {
    units.sort_by_key(|u| u.pos);

    let mut i = 0;
    'unit: while i < units.len() {
        let unit = units[i];

        if units.iter().find(|u| u.team != unit.team) == None {
            return false;
        }

        if let Some(index) = try_attack(map, units, unit) {
            units[index].hp -= 3;
            if units[index].hp <= 0 {
                units.remove(index);
                if index > i {
                    i += 1;
                }
                continue 'unit;
            }
            i += 1;
            continue 'unit;
        }

        let dist_map = get_dist_map(&map, &units, unit.pos);

        let tile = possible_attack_tiles(&map, &units, unit.team)
            .into_iter()
            .map(|p| (dist_map.get(&p), p))
            .filter(|(d, p)| d.is_some())
            .min();

        if let Some((_, tile)) = tile {
            let enemy_dist_map = get_dist_map(&map, &units, tile);
            let (_, move_to) = unit.pos.adjacent()
                .into_iter()
                .filter(|&p| tile_clear(map, units, p))
                .map(|p| (enemy_dist_map.get(&p), p))
                .filter(|(d, p)| d.is_some())
                .min()
                .unwrap();

            units[i].pos = move_to;
        }

        let unit = units[i];

        if let Some(index) = try_attack(map, units, unit) {
            units[index].hp -= 3;
            if units[index].hp <= 0 {
                units.remove(index);
                if index > i {
                    i += 1;
                }
                continue 'unit;
            }
            i += 1;
            continue 'unit;
        }
        i += 1;
    }

    true
}

fn main() {
    let stdin = io::stdin();
    let mut map = HashMap::new();
    let mut units = Vec::new();

    for (y, line) in stdin.lock().lines().map(|x| x.unwrap()).enumerate() {
        for (x, c) in line.chars().enumerate().filter(|(_,c)| !c.is_whitespace()) {
            let x = x as i32;
            let y = y as i32;

            let point = Point {x, y};
            if c == '#' {
                map.insert(point, '#');
            } else if c == '.' {
                map.insert(point, '.');
            } else {
                map.insert(point, '.');
                units.push(Unit::new(point, c));
            }
        }
    }

    print(&map, &units);
    let mut rounds = 0;
    while round(&map, &mut units) {
        rounds += 1;
        print(&map, &units);
        println!("\n");
    }
    print(&map, &units);
    for unit in units.iter() {
        println!("{}", unit.hp);
    }
    println!("{} * {}", rounds, units.iter().map(|u| u.hp).sum::<i32>());
    println!("{}", rounds * units.iter().map(|u| u.hp).sum::<i32>());

}

fn print(map: &HashMap<Point, char>, units: &Vec<Unit>) {
    let maxy = map.keys().max_by_key(|k| k.y).unwrap().y;
    let maxx = map.keys().max_by_key(|k| k.x).unwrap().x;
    for y in 0..=maxy {
        for x in 0..=maxx {
            let point = Point { x, y};
            if let Some(unit) = units.iter().find(|u| u.pos == point) {
                print!("{}", unit.team);
            } else if let Some(tile) = map.get(&point) {
                print!("{}", tile);
            }
        }
        println!();
    }
    println!();
}
