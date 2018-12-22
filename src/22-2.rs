use std::io::{self, BufRead};
use std::collections::HashMap;
use std::collections::hash_map::Entry;
use std::collections::VecDeque;

const ROCKY: i32 = 0;
const WET: i32 = 1;
const NARROW: i32 = 2;

#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn new(s: &str) -> Point {
        let mut it = s
            .split(|c: char| !c.is_ascii_digit())
            .filter(|x| !x.is_empty())
            .map(|x| x.parse::<i32>().unwrap());

        Point {
            x: it.next().unwrap(),
            y: it.next().unwrap(),
        }
    }

    fn adjacent(self) -> Vec<Point> {
        vec![
            Point{ x: self.x, y: self.y - 1},
            Point{ x: self.x - 1, y: self.y},
            Point{ x: self.x + 1, y: self.y},
            Point{ x: self.x, y: self.y + 1},
        ]
    }

    fn left(&self) -> Point {
        Point{ x: self.x - 1, y: self.y }
    }

    fn up(&self) -> Point {
        Point{ x: self.x, y: self.y - 1}
    }
}

#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq)]
enum Tool {
    Torch,
    Climbing,
    Neither,
}

impl Tool {
    fn can_step(self, tile: i32) -> bool {
        match self {
            Tool::Neither => tile != ROCKY,
            Tool::Torch => tile != WET,
            Tool::Climbing => tile != NARROW,
        }
    }

    fn new_tool(tool: Tool, from: i32, to: i32) -> Tool {
        if from == to {
            return tool;
        }

        for &tool in [Tool::Torch, Tool::Climbing, Tool::Neither].into_iter() {
            if tool.can_step(from) && tool.can_step(to) {
                return tool;
            }
        }

        panic!("did not pick a tool");
    }
}

#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq)]
struct Step {
    tool: Tool,
    pos: Point,
    dist: i32,
}

impl Default for Step {
    fn default() -> Step {
        Step {
            tool: Tool::Torch,
            pos: Point { x:0, y:0 },
            dist: 0,
        }
    }
}

fn risk(point: Point, target: Point, depth: i32, geologic_map: &mut HashMap<Point, i32>) -> i32 {
    erosion_level(point, target, depth, geologic_map) % 3
}

fn erosion_level(point: Point, target: Point, depth: i32, geologic_map: &mut HashMap<Point, i32>) -> i32 {
    (geologic_index(point, target, depth, geologic_map) + depth) % 20183
}

fn geologic_index(point: Point, target: Point, depth: i32, geologic_map: &mut HashMap<Point, i32>) -> i32 {

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

fn get_tile(point: Point, target: Point, depth: i32, geologic_map: &mut HashMap<Point, i32>) -> i32 {
    let v = risk(point, target, depth, geologic_map);
    ((v + depth) % 20183) % 3
}

fn next(step: Step, target: Point, depth: i32, map: &mut HashMap<Point, i32>) -> i32 {
    let mut queue = VecDeque::new();
    let mut seen = HashMap::new();
    let mut min = 957;
    queue.push_back(step);
    seen.insert((step.pos, step.tool), 0);

    while let Some(step) = queue.pop_front() {
        if step.dist > min {
            continue;
        }

        if step.pos == target {
            min = step.dist;
            continue;
        }

        for pos in step.pos.adjacent() {
            if pos.x < 0 || pos.x > depth || pos.y < 0 || pos.y > depth {
                continue;
            }

            let tool = Tool::new_tool(step.tool, get_tile(step.pos, target, depth, map), get_tile(pos, target, depth, map));
            let mut dist = step.dist + if tool == step.tool {
                1
            } else {
                8
            };

            if pos == target && tool != Tool::Torch {
                dist += 7;
            }

            let step = Step{ pos, dist, tool };
            match seen.entry((step.pos, step.tool)) {
                Entry::Vacant(mut v) => {
                    v.insert(step.dist);
                },
                Entry::Occupied(mut v) => if step.dist < *v.get() {
                    v.insert(step.dist);
                } else {
                    continue;
                },
            }

            queue.push_back(step);
        }
    }

    min
}

fn main() {
    let stdin = io::stdin();
    let lines = stdin.lock().lines().map(|x| x.unwrap()).collect::<Vec<_>>();
    let depth = lines[0]
        .split(|c: char| !c.is_ascii_digit())
        .filter(|x| !x.is_empty())
        .map(|x| x.parse::<i32>().unwrap())
        .next()
        .unwrap();
    let target = Point::new(&lines[1]);

    let mut geologic_map = HashMap::new();

    let min = next(Step::default(), target, depth, &mut geologic_map);
    println!("{}", min);
}

