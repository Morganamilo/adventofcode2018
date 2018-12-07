use std::collections::BTreeMap;
use std::io::{self, BufRead};

#[derive(Copy, Clone, Debug)]
struct Step {
    name: char,
    depends: char,
}

impl Step {
    fn new(s: &str) -> Step {
        let mut words = s.split_whitespace().skip(1);

        Step {
            depends: words.next().unwrap().chars().next().unwrap(),
            name: words.skip(5).next().unwrap().chars().next().unwrap(),
        }
    }
}

#[derive(Clone, Debug)]
struct Package {
    name: char,
    depends: Vec<char>,
}

impl Package {
    fn new(name: char, depends: Vec<char>) -> Package {
        Package { name, depends }
    }
}

fn main() {
    let stdin = io::stdin();
    let mut bt = BTreeMap::new();

    for step in stdin.lock().lines().map(|x| Step::new(&x.unwrap())) {
        bt.entry(step.name).or_insert(vec![]).push(step.depends);
        bt.entry(step.depends).or_insert(vec![]);
    }

    let mut packages = bt
        .into_iter()
        .map(|(k, v)| Package::new(k, v))
        .collect::<Vec<_>>();

    while let Some(index) = packages.iter().position(|p| p.depends.len() == 0) {
        let task = packages.remove(index).name;
        print!("{}", task);

        for package in &mut packages {
            package.depends.retain(|&c| c != task);
        }
    }

    println!();
}
