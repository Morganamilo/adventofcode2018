use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut double_counter = 0;
    let mut tripple_counter = 0;

    for word in stdin.lock().lines().map(|l| l.unwrap()) {
        let mut letters = [0; 26];
        word.chars().map(|c| c as usize - 'a' as usize).for_each(|c| letters[c] += 1);
        double_counter += letters.iter().any(|&x| x == 2) as i32;
        tripple_counter += letters.iter().any(|&x| x == 3) as i32;
    }

    println!("{}", double_counter * tripple_counter);
}
