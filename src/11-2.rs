use std::io::{self, Read};

type Grid = [[i32; 300]; 300];

fn calculate_cell(x: i32, y: i32, serial: i32) -> i32 {
    let mut ret;
    let rackid = x + 10;

    ret = rackid * y;
    ret += serial;
    ret *= rackid;
    ret = (ret / 100) % 10;
    ret - 5
}

fn expand_area(x: usize, y: usize, size: usize, cells: &Grid) -> i32 {
    let mut total = 0;

    let endy = y+size-1;
    let endx = x+size-1;

    for x in x..endx {
        total += cells[x][endy];
    }

    for y in y..endy{
        total += cells[endx][y];
    }

    total += cells[endx][endy];
    total
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let serial = input.trim().parse::<i32>().unwrap();

    let mut max = (0, 0, 0, 0);
    let mut cells = [[0; 300]; 300];
    let mut totals = [[0; 300]; 300];

    for y in 0..300 {
        for x in 0..300 {
            cells[x][y] = calculate_cell(x as i32, y as i32, serial);
        }
    }

    for size in 1..=300 {
        for y in 0..300-size+1 {
            for x in 0..300-size+1 {
                totals[x][y] += expand_area(x, y, size, &cells);
                if totals[x][y] > max.2 {
                    max = (x, y, totals[x][y], size);
                }
            }
        }
    }

    println!("{},{},{}", max.0, max.1, max.3);
}
