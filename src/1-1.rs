use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let res = stdin.lock().lines().fold(0, |t, f| t + f.unwrap().parse::<i32>().unwrap());
    println!("{}", res);
}
