extern crate advent_of_code;
extern crate failure;
extern crate regex;
extern crate itertools;

use regex::Regex;

use advent_of_code::Result;

use itertools::Either;

use std::fs;
use std::io::prelude::*;
use std::collections::HashMap;

type Map = HashMap<String, (u32, Option<String>, Vec<String>)>;

fn get_data(path: &str) -> Result<Map> {
    let mut f = fs::File::open(path)?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    let mut map = HashMap::new();
    let rx = Regex::new(r"(\w+) \((\d+)\)(?: -> ((?:(?:\w+)(?:, )?)+))?")?;
    s.lines()
        .for_each(|line| {
            let m = rx.captures(line).unwrap();
            let name = m.get(1).unwrap().as_str().to_string();
            let weight = m.get(2).unwrap().as_str().parse::<u32>().unwrap();
            let children = match m.get(3) {
                Some(children) => children.as_str().split(", ").map(String::from).collect(),
                None => Vec::new()
            };
            assert!(map.insert(name, (weight, None, children)).is_none());
        });
    Ok(map)
}

fn add_parents(map: &mut Map) {
    let elems: Vec<_> = map.iter().map(|(k, v)| (k.clone(), v.clone())).collect();
    for &(ref k, (_, _, ref children)) in elems.iter() {
        for child in children {
            match map.get_mut(child) {
                Some(&mut (_, ref mut parent, _)) => *parent = Some(k.clone()),
                None => panic!("Not found: {}", child)
            }
        }
    }
}
fn get_root(map: &Map) -> String {
    let mut name = map.keys().nth(1).unwrap().to_string();
    loop {
        let (_, ref parent, _) = map[&name];
        if let &Some(ref parent) = parent {
            name = parent.to_string();
        } else {
            return name
        }
    }
}

fn circus_v1(map: &mut Map) -> String {
    add_parents(map);
    get_root(map)
}

fn get_node_weight(node: &str, map: &Map) -> Either<(u32, u32), u32> {
    let (ref weight, _, ref children) = map[node];
    let mut child_weights = Vec::new();
    for c in children {
        match get_node_weight(c, map) {
            Either::Left((sum, w)) => child_weights.push((sum, w)),
            Either::Right(w) => return Either::Right(w)
        }
    }
    if child_weights.len() > 2 {
        // check they are even
        let mut map = HashMap::new();
        for &(s, w) in &child_weights {
            let v = map.entry(s + w).or_insert(0);
            *v += 1
        }
        if map.len() > 1 {
            let mut kvs: Vec<_> = map.iter().collect();
            kvs.sort_by_key(|&(_, v)| v);
            let good_weight = *kvs[1].0;
            let bad_weight = *kvs[0].0;
            let diff = good_weight as i32 - bad_weight as i32;
            for (s, w) in child_weights {
                if s + w == bad_weight {
                    let res = w as i32 + diff;
                    return Either::Right(res as u32);
                }
            }
            unreachable!()
        }
    }
    let sum: u32 = child_weights.iter().map(|&(s, w)| s + w).sum();
    Either::Left((sum, *weight))
}

fn circus_v2(map: &mut Map) -> u32 {
    add_parents(map);
    let root = get_root(map);
    match get_node_weight(&root, map) {
        Either::Left(_) => panic!("Failed to find weight"),
        Either::Right(v) => v
    }
}


fn run() -> Result<()> {
    let mut data = get_data("data/07.txt")?;
    let outcome1 = circus_v1(&mut data);
    println!("v1: {}", outcome1);
    let mut data = get_data("data/07.txt")?;
    let outcome2 = circus_v2(&mut data);
    println!("v2: {}", outcome2);
    Ok(())
}

fn main() {
    run().unwrap_or_else(|e| {
        for cause in e.causes() {
            println!("{}", cause)
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cases_v2() {
        let mut data = get_data("data/07_test.txt").unwrap();
        let outcome2 = circus_v2(&mut data);
        assert_eq!(outcome2, 60)
    }
}
