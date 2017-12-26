extern crate regex;

use regex::Regex;
use std::collections::{HashMap, HashSet};
use std::io::{self, BufRead};

struct Tower {
    id: String,
    weight: i32,
    subtowers: Vec<String>,
}

fn read_towers(reader: &mut BufRead) -> Vec<Tower> {
    let mut towers: Vec<Tower> = Vec::new();
    let re = Regex::new(r"^(\w+)\s+\((\d+)\)(?:\s+->\s+(.*))?").unwrap();

    for line in reader.lines() {
        let line = line.unwrap();
        if let Some(caps) = re.captures(&line) {
            let mut subtowers: Vec<String> = match caps.get(3) {
                Some(subtowers_cap) => {
                    subtowers_cap.as_str()
                        .split_whitespace()
                        .map(|s| String::from(s.trim_matches(',')))
                        .collect()
                }
                None => Vec::new(),
            };

            towers.push(Tower {
                id: String::from(caps.get(1).unwrap().as_str()),
                weight: caps.get(2).unwrap().as_str().parse::<i32>().unwrap(),
                subtowers: subtowers,
            });
        }
    }

    towers
}

fn build_tree(towers: &mut Vec<Tower>) -> (HashMap<String, Tower>, String) {
    let mut tower_map: HashMap<String, Tower> = HashMap::new();
    let root_id = find_root(towers);

    while let Some(tower) = towers.pop() {
        tower_map.insert(tower.id.clone(), tower);
    }

    (tower_map, root_id)
}


fn find_root(towers: &Vec<Tower>) -> String {
    let mut subtowers: HashSet<String> = HashSet::new();

    for tower in towers.iter() {
        for subtower in tower.subtowers.iter() {
            subtowers.insert(subtower.clone());
        }
    }

    towers.iter().find(|n| !subtowers.contains(&n.id)).unwrap().id.clone()
}

struct Balance {
    own_weight: i32,
    total_weight: i32,
    corrected_weight: Option<i32>,
}

fn find_imbalance(towers: &HashMap<String, Tower>, tower_id: &String) -> Balance {
    let current_tower = towers.get(tower_id).unwrap();
    let own_weight = current_tower.weight;

    if current_tower.subtowers.is_empty() {
        return Balance {
            own_weight,
            total_weight: own_weight,
            corrected_weight: None,
        };
    }

    let subtree_balances: Vec<Balance> = current_tower.subtowers
        .iter()
        .map(|sub_id| find_imbalance(towers, sub_id))
        .collect();

    let total_weight = subtree_balances.iter()
        .fold(current_tower.weight, |sum, sb| sum + sb.total_weight);
    let corrected_weight = find_fixed_weight(&subtree_balances);

    Balance { own_weight, total_weight, corrected_weight }
}

fn find_fixed_weight(balances: &Vec<Balance>) -> Option<i32> {
    if balances.len() < 3 {
        return None;
    }

    if let Some(imbalance) = balances.iter().find(|b| b.corrected_weight.is_some()) {
        return imbalance.corrected_weight;
    }

    let weight1 = balances[0].total_weight;

    if weight1 != balances[1].total_weight && weight1 != balances[2].total_weight {
        return Some(fix_weight(&balances[0], balances[1].total_weight));
    }

    if let Some(wrong_balance) = balances.iter().find(|b| b.total_weight != weight1) {
        return Some(fix_weight(&wrong_balance, weight1));
    }

    None
}

fn fix_weight(balance: &Balance, correct_total_weight: i32) -> i32 {
    assert!(balance.total_weight != correct_total_weight);
    balance.own_weight - (balance.total_weight - correct_total_weight)
}

fn main() {
    let stdin = io::stdin();

    let mut towers = read_towers(&mut stdin.lock());
    let (tree, root) = build_tree(&mut towers);
    let Balance {
        own_weight: _,
        total_weight: _, corrected_weight } = find_imbalance(&tree, &root);

    println!("{}", corrected_weight.unwrap());
}
