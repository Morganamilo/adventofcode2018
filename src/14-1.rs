use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let input = input.trim().parse::<usize>().unwrap();

    let mut recipies = Vec::with_capacity(input + 11);
    recipies.push(3);
    recipies.push(7);
    let mut elf1 = 0;
    let mut elf2 = 1;

    while recipies.len() < input + 10 {
        let combined = recipies[elf1] + recipies[elf2];
        if combined > 9 {
            recipies.push(combined / 10);
            recipies.push(combined % 10);
        } else {
            recipies.push(combined);
        }

        elf1 = (elf1 + recipies[elf1] + 1) % recipies.len();
        elf2 = (elf2 + recipies[elf2] + 1) % recipies.len();
    }

    let score = &recipies[input..input+10]
        .iter()
        .map(|c| c.to_string())
        .collect::<String>();

    println!("{}", score);
}

