use std::collections::HashMap;
use std::io::{self, BufRead};

#[derive(Copy, Clone, Debug)]
enum Action {
    Wake,
    Fall,
    Start(i32),
}

impl Action {
    fn new(word: &str, id: &str) -> Action {
        match word {
            "wakes" => Action::Wake,
            "falls" => Action::Fall,
            _ => Action::Start(id.parse().unwrap()),
        }
    }
}

#[derive(Copy, Clone, Debug)]
struct Entry {
    date: i32,
    time: i32,
    action: Action
}

impl Entry {
    fn new(line: &str) -> Entry {
        let word = &mut line
            .split(|c: char| !c.is_alphanumeric())
            .filter(|x| !x.is_empty());

        Entry {
            date: word.take(3).collect::<Vec<_>>().concat().parse().unwrap(),
            time: word.skip(1).next().unwrap().parse().unwrap(),
            action: Action::new(word.next().unwrap(), word.next().unwrap()),
        }
    }
}

fn times(entries: &Vec<Entry>) -> Vec<(i32, i32, i32)>{
    let mut id = 0;
    let mut time = 0;
    let mut awake = true;
    let mut vec = Vec::new();

    for entry in entries {
        match entry.action {
            Action::Start(num) => {
                id = num;
                awake = true;
            },
            Action::Wake if !awake => {
                vec.push((id, time, entry.time));
                awake = true;
            },
            Action::Fall if awake  => {
                awake = false;
                time = entry.time;
            },
            _ => {},
        }
    }

    vec
}

fn main() {
    let stdin = io::stdin();
    let mut hm = HashMap::new();

    let mut entries = stdin.lock().lines().map(|x| Entry::new(&x.unwrap())).collect::<Vec<_>>();
    entries.sort_by(|a, b| a.date.cmp(&b.date).then(a.time.cmp(&b.time)));
    let times = times(&entries);

    for &(id, time, entry_time) in times.iter() {
        *hm.entry(id).or_insert(0) += entry_time - time;
    }

    let (&max_id, _) = hm.iter().max_by_key(|a| a.1).unwrap();
    hm.clear();

    for &(_, time, entry_time) in times.iter().filter(|x| x.0 == max_id) {
        for t in time..entry_time {
            *hm.entry(t).or_insert(0) += 1;
        }
    }

    let (&max_min, _) = hm.iter().max_by_key(|a| a.1).unwrap();
    println!("{}", max_id * max_min);
}
