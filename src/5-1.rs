use std::io::{self, Read};

fn main() {
    let mut out = vec!['-'];
    let mut polymers = String::new();
    io::stdin().read_to_string(&mut polymers).unwrap();
    polymers.pop();

    for letter in polymers.chars()  {
        let last = out[out.len() - 1];

        if letter == last || !letter.eq_ignore_ascii_case(&last) {
            out.push(letter);
        } else {
            out.pop();
        }
    }

    println!("{}", out.len() - 1);
}
