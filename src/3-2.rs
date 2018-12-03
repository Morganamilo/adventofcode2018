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

    let fabrics = stdin.lock()
        .lines()
        .map(|x| Fabric::new(&x.unwrap()))
        .collect::<Vec<_>>();

    for f in fabrics.iter() {
        for i in f.x..f.x + f.w {
            for j in f.y..f.y + f.h {
                arr[i as usize][j as usize] += 1
            }
        }
    }

    'next: for f in fabrics.iter() {
        for i in f.x..f.x + f.w {
            for j in f.y..f.y + f.h {
                if arr[i as usize][j as usize] > 1 {
                    continue 'next;
                }

            }
        }

        println!("{}", f.id);
        return;
    }
}
