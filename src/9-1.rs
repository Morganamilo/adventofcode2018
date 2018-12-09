use std::io::{self, Read};
use std::collections::VecDeque;

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut nums = input
        .split(|c: char| !c.is_ascii_digit())
        .filter(|x| !x.is_empty())
        .map(|x| x.parse().unwrap());

    let player_count = nums.next().unwrap();
    let biggest_marble = nums.next().unwrap();
    let mut circle = VecDeque::with_capacity(biggest_marble);
    let mut players = vec![0; player_count as usize];
    circle.push_back(0);

    for (player, marble) in (0..player_count).cycle().zip(1..=biggest_marble) {
        if marble % 23 == 0 {
            for _ in 0..7 {
                let back = circle.pop_back().unwrap();
                circle.push_front(back);
            }
            players[player] += marble + circle.pop_front().unwrap();
        } else {
            for _ in 0..2 {
                let front = circle.pop_front().unwrap();
                circle.push_back(front);
            }
            circle.push_front(marble);
        }
    }

    println!("{}", players.iter().max().unwrap());
}
