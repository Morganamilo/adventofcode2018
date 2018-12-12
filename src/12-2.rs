use std::io::{self, BufRead};
use std::mem::swap;

fn parse_state(s: &str) -> Vec<bool> {
    s
        .chars()
        .filter(|&c| c == '#' || c == '.')
        .map(|c| c == '#')
        .collect::<Vec<_>>()
}

fn parse_rule(s: &str) -> Option<Vec<bool>> {
    let mut state = parse_state(s);
    if !state.pop().unwrap() {
        return None
    }

    Some(state)
}

fn matches_rules(window: &[bool], rules: &[Vec<bool>]) -> bool {
    rules.iter().any(|r| window == r.as_slice())
}

fn apply_rules(plants: &[bool], new_plants: &mut Vec<bool>, rules: &[Vec<bool>]) {
    let mut window = [false; 5];
    let len = plants.len() as isize;

    for i in -1..=plants.len() as isize {
        (i-2..=i+2)
            .map(|j| if j < 0 || j > len- 1 { false } else { plants[j as usize] })
            .enumerate()
            .for_each(|(i, p)| window[i] = p);

        new_plants.push(matches_rules(&window, rules));
    }
}

fn total_sum(plants: &[bool], generation: isize) -> isize {
    plants
        .iter()
        .enumerate()
        .filter(|&(_, p)| *p)
        .map(|(i, _)| i as isize - generation)
        .sum()
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines().map(|x| x.unwrap());
    let mut plants = parse_state(&lines.next().unwrap());
    let mut new_plants = Vec::with_capacity(plants.len() * 10);

    let rules = lines
        .skip(1)
        .filter_map(|x| parse_rule(&x))
        .collect::<Vec<_>>();


    let mut total = None;
    let mut diff = None;


    for n in 1.. {
        apply_rules(&plants, &mut new_plants, &rules);
        swap(&mut plants, &mut new_plants);
        new_plants.clear();
        let curr_total = total_sum(&plants, n);

        if let Some(total) = total {
            let curr_diff = curr_total - total;

            if let Some(diff) = diff {
                if diff == curr_diff {
                    println!("{}", curr_total + (curr_diff * (5000000000 - n)));
                    break;
                }
            }

            diff = Some(curr_diff);
        }

        total = Some(curr_total);
    }
}
