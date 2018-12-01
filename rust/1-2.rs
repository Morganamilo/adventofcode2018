use std::io::{self, BufRead};
use std::collections::HashSet;

fn main() {
    let mut seen = HashSet::new();
    let mut total = 0;
    let stdin = io::stdin();
    let freqs = stdin.lock().lines().map(|x| x.unwrap().parse::<i32>().unwrap()).collect::<Vec<_>>();

    for freq in freqs.iter().cycle() {
        seen.insert(total);
        total += freq;

        if seen.contains(&total) {
            println!("{}", total);
            return;
        }
    }
}
