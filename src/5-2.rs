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

    out.pop();

    let min: usize = (b'a'..=b'z')
        .map(|x| get_reduced_count(&out, x as char))
        .min()
        .unwrap();

    println!("{}", min);
}

fn get_reduced_count(polymers: &[char], remove: char) -> usize {
    let mut out = vec!['-'];

    for letter in polymers.iter().cloned().filter(|c| !remove.eq_ignore_ascii_case(c)) {
        let last = out[out.len() - 1];

        if letter == last || !letter.eq_ignore_ascii_case(&last) {
            out.push(letter);
        } else {
            out.pop();
        }
    }

    out.len() - 1
}
