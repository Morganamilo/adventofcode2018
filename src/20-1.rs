use std::io::{self, Read};

fn opposite(c: char) -> char {
    match c {
        'N' => 'S',
        'E' => 'W',
        'S' => 'N',
        'W' => 'E',
        _ => panic!(),
    }
}

fn extend_or_backtrack(s: &mut String, c: char) {
    if s.ends_with(opposite(c)) {
        s.pop();
    } else {
        s.push(c);
    }
}

fn parse(s: &str) -> Option<usize> {
    let mut stack = vec![vec![String::new()]];

    for c in s.chars() {
        match c {
            'N' | 'E' | 'S' | 'W' => {
                let mut last = stack.last_mut()?.last_mut()?;
                extend_or_backtrack(&mut last, c);
            },
            ')' => {
                let max = stack.pop()?.into_iter().max_by_key(|s| s.len())?;
                let mut last = stack.last_mut()?.last_mut()?;
                for c in max.chars() {
                    extend_or_backtrack(&mut last, c);
                }
            },
            '|' => stack.last_mut()?.push(String::new()),
            '(' => stack.push(vec![String::new()]),
            _ => {},
        };
    }

    Some(stack[0].iter().map(|s| s.len()).max()?)
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    println!("{}", parse(&input).unwrap());
}
