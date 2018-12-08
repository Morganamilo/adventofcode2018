use std::io::{self, Read};

fn parse_node(mut data: &[usize]) -> (usize, &[usize]) {
    let mut total = 0;
    let children = data[0];
    let metadata = data[1];
    data = &data[2..];

    for _ in 0..children {
        let next = parse_node(data);
        total += next.0;
        data = next.1;
    }

    total += data[..metadata].iter().sum::<usize>();
    (total, &data[metadata..])
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let nums = input.split_whitespace().map(|x| x.parse().unwrap()).collect::<Vec<usize>>();

    let (total, _) = parse_node(&nums);
    println!("{}", total);
}
