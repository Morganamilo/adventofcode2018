use std::collections::BTreeMap;
use std::io::{self, BufRead};

const WORKER_COUNT: usize = 5;
const LETTER_TIME: i32 = 60;

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

#[derive(Debug)]
struct Package {
    name: char,
    depends: Vec<char>,
}

impl Package {
    fn new(name: char, depends: Vec<char>) -> Package {
        Package { name, depends }
    }
}

#[derive(Copy, Clone, Debug)]
struct Worker {
    has: char,
    waiting: i32,
}

impl Default for Worker {
    fn default() -> Worker {
        Worker {
            has: '.',
            waiting: 0,
        }
    }
}

fn main() {
    let stdin = io::stdin();
    let mut bt = BTreeMap::new();
    let workers = &mut [Worker::default(); WORKER_COUNT];

    for step in stdin.lock().lines().map(|x| Step::new(&x.unwrap())) {
        bt.entry(step.depends).or_insert(vec![]);
        bt.entry(step.name).or_insert(vec![]).push(step.depends);
    }

    let mut packages = bt
        .into_iter()
        .map(|(k, v)| Package::new(k, v))
        .collect::<Vec<_>>();

    for time in 0.. {
        for worker in workers.iter_mut().filter(|w| w.waiting == 0) {
            if let Some(index) = packages.iter().position(|p| p.depends.is_empty()) {
                let task = packages.remove(index).name;
                worker.has = task;
                worker.waiting = task as i32 - 'A' as i32 + LETTER_TIME + 1;
            }
        }

        if workers.iter().all(|w| w.waiting == 0) {
            println!("{}", time);
            break;
        }

        for worker in workers.iter_mut().filter(|w| w.waiting != 0) {
            worker.waiting -= 1;

            if worker.waiting == 0 {
                for package in packages.iter_mut() {
                    package.depends.retain(|&c| c != worker.has);
                }
            }
        }
    }
}
