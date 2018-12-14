use std::io::{self, Read};

fn find(input: &[usize], recipies: &[usize]) -> bool {
    let len = recipies.len();
    let inlen = input.len();

    if len >= inlen && &recipies[len-inlen..] == input {
        return true;
    }

    false
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let input = input
        .trim()
        .chars()
        .map(|c| c.to_digit(10).unwrap() as usize)
        .collect::<Vec<_>>();

    let mut recipies = Vec::with_capacity(25_000_000);
    recipies.push(3);
    recipies.push(7);

    let mut elf1 = 0;
    let mut elf2 = 1;

    loop {
        let combined = recipies[elf1] + recipies[elf2];

        if combined > 9 {
            recipies.push(combined / 10);

            if find(&input, &recipies) {
                    println!("{}", recipies.len() - input.len());
                    break;
            }

            recipies.push(combined % 10);
        } else {
            recipies.push(combined);
        }

        if find(&input, &recipies) {
            println!("{}", recipies.len() - input.len());
            break;
        }

        elf1 = (elf1 + recipies[elf1] + 1) % recipies.len();
        elf2 = (elf2 + recipies[elf2] + 1) % recipies.len();
    }
}

