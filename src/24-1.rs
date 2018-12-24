use std::io::{self, BufRead};
use std::cmp::Ordering;
use std::cell::RefCell;

#[derive(Clone, Debug, Hash, Eq, PartialEq)]
struct Group {
    team: String,
    units: i32,
    hp: i32,
    weak_to: Vec<String>,
    immune_to: Vec<String>,
    attacks_with: String,
    damage: i32,
    initiative: i32,
}

impl PartialOrd for Group {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Group {
    fn cmp(&self, other: &Self) -> Ordering {
        other.effective_power().cmp(&self.effective_power())
            .then(other.initiative.cmp(&self.initiative))
    }
}

impl Group {
    fn new(team: &str, s: &str) -> Group {
        let mut words = s.split_whitespace();
        let mut weak;
        let mut switch = false;
        let mut weak_to = vec![];
        let mut immune_to = vec![];
        let mut end = false;

        let team = team.into();
        let units = words.next().unwrap().parse().unwrap();
        let hp =  words.nth(3).unwrap().parse().unwrap();
        let open = words.nth(2).unwrap();
        if open.starts_with('(') {
            weak = open.trim_start_matches('(') == "weak";
            words.nth(0);
            while !end {
                let mut word = words.next().unwrap().trim_end_matches(',');

                if switch {
                    weak = !weak;
                    switch = false;
                }

                if word.ends_with(';') {
                    word = word.trim_end_matches(';');
                    words.nth(1);
                    switch = true;
                }

                if word.ends_with(')') {
                    word = word.trim_end_matches(')');
                    end = true;
                }

                if weak {
                    weak_to.push(word.into());
                } else {
                    immune_to.push(word.into());
                }
            }
            words.nth(0);
        }

        words.nth(3);
        let damage = words.next().unwrap().parse().unwrap();
        let attacks_with = words.next().unwrap().into();
        let initiative = words.last().unwrap().parse().unwrap();

        Group {
            team,
            units,
            hp,
            weak_to,
            immune_to,
            attacks_with,
            damage,
            initiative,
        }
    }

    fn effective_power(&self) -> i32 {
        self.units * self.damage
    }

    fn damage_dealt(&self, other: &Self) -> i32 {
        if other.immune_to.contains(&self.attacks_with) {
            0
        } else if other.weak_to.contains(&self.attacks_with) {
            self.effective_power() * 2
        } else {
            self.effective_power()
        }
    }
}

fn battle(groups: &mut Vec<RefCell<Group>>) -> i32 {
    loop {
        {
            let mut targets = vec![];
            groups.sort();
            let mut target_pool = groups.iter().collect::<Vec<_>>();

            for rc in groups.iter() {
                let group = rc.borrow();
                target_pool.sort_by(|a, b| {
                    let a = a.borrow();
                    let b = b.borrow();
                    group.damage_dealt(&b).cmp(&group.damage_dealt(&a))
                        .then(b.effective_power().cmp(&a.effective_power()))
                        .then(b.initiative.cmp(&a.initiative))
                });

                if let Some(i) = target_pool.iter().position(|x| x.borrow().team != group.team) {
                    if group.damage_dealt(&target_pool[i].borrow()) > 0 {
                        targets.push((rc, target_pool.remove(i)));
                    }
                }
            }
            targets.sort_by(|a,b| b.0.borrow().initiative.cmp(&a.0.borrow().initiative));

            for (attack, defend) in targets.iter() {
                if attack.borrow().hp <= 0 {
                    continue;
                }
                let damage = attack.borrow().damage_dealt(&defend.borrow());
                let mut defend = defend.borrow_mut();
                println!("damage {:8}, units {:8}, hp {:8}, killed {:8}", damage, defend.units, defend.hp, defend.units.min(damage / defend.hp));
                defend.units -= defend.units.min(damage / defend.hp);
            }
        }
        groups.retain(|g| g.borrow().units > 0);

        if groups.iter().find(|x| x.borrow().team.as_str() == "infection").is_none()
            ||  groups.iter().find(|x| x.borrow().team.as_str() == "immune").is_none() {
            break
        }

        println!();
    }

    groups.iter().map(|x| x.borrow().units).sum::<i32>()
}

fn main() {
    let stdin = io::stdin();
    let lines = stdin.lock()
        .lines()
        .map(|x| x.unwrap())
        .collect::<Vec<_>>();

    let mut split = lines.splitn(2, |l| l == "");
    let mut groups = Vec::new();

    split.next().unwrap()
        .iter()
        .filter(|l| l.starts_with(|c: char| c.is_digit(10)))
        .map(|l| Group::new("immune", l))
        .for_each(|x| groups.push(RefCell::new(x)));


    split.next().unwrap()
        .iter()
        .filter(|l| l.starts_with(|c: char| c.is_digit(10)))
        .map(|l| Group::new("infection", l))
        .for_each(|x| groups.push(RefCell::new(x)));

    println!("{}",  battle(&mut groups));
}
