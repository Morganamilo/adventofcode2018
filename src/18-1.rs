use std::io::{self, BufRead};

type Grid = [[char; 50]; 50];

fn count(tiles: &Grid, x: usize, y: usize) -> (usize, usize, usize) {
    let mut chars = String::new();

    for i in (x).max(1)-1..(x+2).min(tiles.len()) {
        for j in (y).max(1)-1..(y+2).min(tiles[0].len()) {
            if i == x && j == y {
                continue
            }

            chars.push(tiles[i][j]);
        }
    }

    ( chars.matches('.').count(), chars.matches('|').count(), chars.matches('#').count() )
}

fn step(tiles: Grid) -> Grid {
    let mut next = tiles.clone();

    for x in 0..tiles.len() {
        for y in 0..tiles[0].len() {
            let (open, trees, lumberyard) = count(&tiles, x, y);
            match tiles[x][y] {
                '.' => {
                    if trees >= 3 {
                        next[x][y] = '|';
                    }
                },
                '|' => {
                    if lumberyard >= 3 {
                        next[x][y] = '#';
                    }
                },
                '#' => {
                    if !(lumberyard >= 1 && trees >= 1) {
                        next[x][y] = '.';
                    }
                },
                _ => panic!("invalid char"),
            }
        }
    }

    next
}

fn main() {
    let stdin = io::stdin();
    let mut tiles = [['.'; 50]; 50];

    for (y, line) in stdin.lock().lines().map(|x| x.unwrap()).enumerate() {
        for (x, c) in line.chars().enumerate() {
            tiles[x][y] = c;
        }
    }

    for _ in 0..10 {
        tiles = step(tiles);
    }


    print(&tiles);

    let trees = tiles.iter().flat_map(|x| x.iter()).filter(|&&c| c == '|').count();
    let lumberyard = tiles.iter().flat_map(|x| x.iter()).filter(|&&c| c == '#').count();
    println!("{}", trees * lumberyard);
}

fn print(tiles: &Grid) {
    for y in 0..tiles.len() {
        for x in 0..tiles[0].len() {
            print!("{}", tiles[x][y]);
        }
        println!();
    }
    println!();
}
