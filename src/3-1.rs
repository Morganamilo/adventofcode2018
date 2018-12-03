use std::io::{self, BufRead};

#[derive(Copy, Clone, Debug)]
struct Fabric {
    id: i32,
    x: i32,
    y: i32,
    w: i32,
    h: i32,
}

impl Fabric {
    fn new(line: &str) -> Fabric {
        let word = &mut line
            .split(|c: char| !c.is_ascii_digit())
            .filter(|x| !x.is_empty())
            .map(|x| x.parse::<i32>().unwrap());

        Fabric {
            id: word.next().unwrap(),
            x: word.next().unwrap(),
            y: word.next().unwrap(),
            w: word.next().unwrap(),
            h: word.next().unwrap(),
        }
    }
}

fn main() {
    let stdin = io::stdin();
    let mut arr = [[0; 1000]; 1000];

    for line in stdin.lock().lines().map(|x| x.unwrap()) {
        let f = Fabric::new(&line);

        for x in f.x..f.x + f.w {
            for y in f.y..f.y + f.h {
                arr[x as usize][y as usize] += 1
            }
        }
    }

    println!("{}", arr.iter().flat_map(|x| x.iter()).filter(|&&x| x > 1).count());
}
