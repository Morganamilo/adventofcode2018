use std::io::{self, BufRead};
use std::collections::{HashMap, HashSet};
use std::cmp::Ordering;
use std::ops::{Add, AddAssign};

#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq, Ord, PartialOrd)]
struct Point {
    y: i32,
    x: i32,
}

impl AddAssign<Direction> for Point {
    fn add_assign(&mut self, other: Direction) {
        match other {
            Direction::Up => self.y -= 1,
            Direction::Down => self.y += 1,
            Direction::Left => self.x -= 1,
            Direction::Right => self.x += 1,
        };
    }
}

impl AddAssign<Track> for Cart {
    fn add_assign(&mut self, other: Track) {
        match other {
            Track::Up => (),
            Track::Right => (),
            Track::Cross => {
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
            },
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

impl Ord for Cart {
    fn cmp(&self, other: &Cart) -> Ordering {
        self.pos.cmp(&other.pos)
    }
}

impl PartialOrd for Cart {
    fn partial_cmp(&self, other: &Cart) -> Option<Ordering> {
        Some(self.cmp(other))
    }
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
}

#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq)]
enum Track {
    Up,
    Right,
    Cross,
    BendLeft,
    BendRight,
}

impl Track {
    fn new(c: char) -> Track {
        match c {
            '|' | 'v' | '^' =>  Track::Up,
            '-' | '<' | '>' => Track::Right,
            '+' => Track::Cross,
            '\\' => Track::BendLeft,
            '/' => Track::BendRight,
            _ => panic!("invalid track {}", c),
        }
    }
}

fn tick(map: &HashMap<Point, Track>, carts: &mut Vec<Cart>) -> Vec<usize> {
    let mut seen: HashMap<Point, usize> = HashMap::with_capacity(carts.len());

    let mut vec = Vec::new();
    let cc = carts.clone();
    carts.sort();
    for i in 0..carts.len() {
        //println!("{:?}", cart);
        {
        let cart = &mut carts[i];
        let &track = map.get(&cart.pos).unwrap();
        *cart += track;
        cart.pos += cart.direction;
        }

        for (j, cart) in carts.iter().enumerate() {
            if i != j && carts[i].pos == cart.pos {
                vec.push(i);
                vec.push(j);
            }
        }


   }

    vec
}

//71,14
//124,103
fn main() {
    let stdin = io::stdin();
    let mut map = HashMap::new();
    let mut carts = Vec::new();

    for (y, line) in stdin.lock().lines().map(|x| x.unwrap()).enumerate() {
        for (x, c) in line.chars().enumerate().filter(|(_,c)| !c.is_whitespace()) {
            let x = x as i32;
            let y = y as i32;
            let point = Point {x, y};

            if let Some(cart) = Cart::new(c, point) {
                carts.push(cart);
                map.insert(point, Track::new(c));
            } else {
                map.insert(point, Track::new(c));
            }
        }
    }

    println!("carts: {}", carts.len());

    let mut points = map.iter().collect::<Vec<_>>();
    points.sort_by_key(|p| p.0);
    for point in points {
        println!("{:?}", point);
    }

    for cart in carts.iter() {
        println!("{:?}", cart);
    }


    //print_map(&map, &carts);
    carts.sort();
    for n in 0.. {
        println!("tick: {} {}", n, carts.len());
        let mut colision = tick(&map, &mut carts);
        colision.sort();
        colision.dedup();
        for i in colision.into_iter().rev() {
            carts.remove(i);
            if carts.len() == 1 {
                println!("{},{}", carts[0].pos.x, carts[0].pos.y);
                return;
            }
        }
        //print_map(&map, &carts, Point{x:0, y:0});

    }
}

fn print_map(map: &HashMap<Point, Track>, carts: &[Cart], pos: Point) {
    let maxy = map.keys().max_by_key(|p| p.y).unwrap();
    let maxx = map.keys().max_by_key(|p| p.x).unwrap();
    for y in 0..=maxy.y {
        for x in 0..=maxx.x {
            let point = Point{x,y};
            let track = map.get(&point);

            if point == pos {
                print!("\x1b[41mX\x1b[0m");
                continue;
            }

            if let Some(cart) = carts.iter().find(|c| c.pos == point) {
                match cart.direction {
                    Direction::Up => print!("\x1b[41m^\x1b[0m"),
                    Direction::Down => print!("\x1b[41mv\x1b[0m"),
                    Direction::Left => print!("\x1b[41m<\x1b[0m"),
                    Direction::Right => print!("\x1b[41m>\x1b[0m"),
                };
                continue;
            }

            if let Some(track) = track {
                let s = match track {
                    Track::Up => "|",
                    Track::Right => "-",
                    Track::Cross => "+",
                    Track::BendLeft => "\\",
                    Track::BendRight => "/",
                    _ => " ",
                };
                print!("{}", s);
            } else {
                print!(" ");
            }

        }
        println!();
    }

}
