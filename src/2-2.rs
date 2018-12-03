use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let input = stdin.lock().lines().map(|x| x.unwrap()).collect::<Vec<_>>();

    for (k, a) in input.iter().enumerate() {
        for b in input.iter().skip(k) {
            let different = a
                .chars()
                .zip(b.chars())
                .filter(|(i, j)| *i != *j)
                .take(2)
                .count();

            if different == 1 {
                let out = a
                    .chars()
                    .zip(b.chars())
                    .filter(|(ci, cj)| *ci == *cj)
                    .map(|(ci, _)| ci)
                    .collect::<String>();

                println!("{}", out);
                return;
            }
        }
    }
}
