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

fn calculate3x3(x: usize, y: usize, cells: &Grid) -> i32 {
    let mut total = 0;

    for y in y..y+3 {
        for x in x..x+3 {
            total += cells[x][y]
        }
    }

    total
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let serial = input.trim().parse::<i32>().unwrap();

    let mut max = (0, 0, 0);
    let mut cells = [[0; 300]; 300];

    for y in 0..300 {
        for x in 0..300 {
            cells[x][y] = calculate_cell(x as i32, y as i32, serial);
        }
    }

    for y in 0..300-2 {
        for x in 0..300-2 {
            let total = calculate3x3(x as usize, y as usize, &cells);
            if total > max.2 {
                max = (x, y, total);
            }
        }
    }

    println!("{},{}", max.0, max.1);
}
