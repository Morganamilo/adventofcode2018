use std::io::{self, BufRead};
use std::collections::{HashMap};

#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq, Ord, PartialOrd)]
struct Point {
    y: usize,
    x: usize,
}

#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn left(&mut self) {
        *self = match self {
            Direction::Up => Direction::Left,
            Direction::Down => Direction::Right,
            Direction::Left => Direction::Down,
            Direction::Right => Direction::Up,
        };
    }

    fn right(&mut self) {
        self.left();
        self.left();
        self.left();
    }
}

#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq)]
struct Cart {
    pos: Point,
    turn: Direction,
    direction: Direction,
}

impl Cart {
    fn new(c: char, pos: Point) -> Option<Cart> {
        let direction = match c {
            '^' => Direction::Up,
            'v' => Direction::Down,
            '<' => Direction::Left,
            '>' => Direction::Right,
            _ => return None,
        };

        let turn = Direction::Left;
        Some(Cart {pos, turn, direction})
    }

    fn step(&mut self) {
        match self.direction {
            Direction::Up => self.pos.y -= 1,
            Direction::Down => self.pos.y += 1,
            Direction::Left => self.pos.x -= 1,
            Direction::Right => self.pos.x += 1,
        };
    }

    fn turn_crossroad(&mut self) {
        match self.turn {
            Direction::Left => self.direction.left(),
            Direction::Up => (),
            Direction::Right => self.direction.right(),
            _ => panic!(),
        };
        self.turn.right();
        if self.turn == Direction::Down {
            self.turn = Direction::Left;
        };
    }

    fn turn(&mut self, track: Track) {
        match track {
            Track::Straight => (),
            Track::Cross => self.turn_crossroad(),
            Track::BendLeft => {
                match self.direction {
                    Direction::Up => self.direction.left(),
                    Direction::Down => self.direction.left(),
                    Direction::Left => self.direction.right(),
                    Direction::Right => self.direction.right(),
                }
            },
            Track::BendRight => {
                match self.direction {
                    Direction::Up => self.direction.right(),
                    Direction::Down => self.direction.right(),
                    Direction::Left => self.direction.left(),
                    Direction::Right => self.direction.left(),
                }
            },
        };
    }
}

#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq)]
enum Track {
    Straight,
    Cross,
    BendLeft,
    BendRight,
}

impl Track {
    fn new(c: char) -> Track {
        match c {
            '|' | 'v' | '^' | '-' | '<' | '>' => Track::Straight,
            '+' => Track::Cross,
            '\\' => Track::BendLeft,
            '/' => Track::BendRight,
            _ => panic!("invalid track {}", c),
        }
    }
}

fn tick(map: &HashMap<Point, Track>, carts: &mut Vec<Cart>) -> Option<Point> {
    carts.sort_by_key(|c| c.pos);
    for i in 0..carts.len() {
        let &track = map.get(&carts[i].pos).unwrap();
        carts[i].turn(track);
        carts[i].step();

        if carts.iter().filter(|c| c.pos == carts[i].pos).take(2).count() > 1 {
            return Some(carts[i].pos);
        }
   }

    None
}

fn read_input() -> (HashMap<Point, Track>, Vec<Cart>) {
    let stdin = io::stdin();
    let mut map = HashMap::new();
    let mut carts = Vec::new();

    for (y, line) in stdin.lock().lines().map(|x| x.unwrap()).enumerate() {
        for (x, c) in line.chars().enumerate().filter(|(_,c)| !c.is_whitespace()) {
            let point = Point {x, y};

            if let Some(cart) = Cart::new(c, point) {
                carts.push(cart);
            }

            map.insert(point, Track::new(c));
        }
    }

    (map, carts)
}

fn main() {
    let (map, mut carts) = read_input();

    loop {
        let colision = tick(&map, &mut carts);
        if let Some(colision) = colision {
            println!("{},{}", colision.x, colision.y);
            break;
        }
    }
}
