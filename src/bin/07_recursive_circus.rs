extern crate advent_of_code;
extern crate failure;
extern crate regex;

use regex::Regex;

use advent_of_code::Result;

use std::fs;
use std::io::prelude::*;
use std::collections::HashMap;

type Map = HashMap<String, (u32, Option<String>, Vec<String>)>;

fn get_data() -> Result<Map> {
    let mut f = fs::File::open("data/07.txt")?;
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

fn circus_v1(map: &mut Map) -> String {
    add_parents(map);
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

fn run() -> Result<()> {
    let mut data = get_data()?;
    let outcome1 = circus_v1(&mut data);
    println!("v1: {}", outcome1);
    // let mut data = get_data()?;
    // let outcome2 = reallocate_v2(&mut data);
    // println!("v2: {}", outcome2);
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

    // #[test]
    // fn cases_v1() {
    //     assert_eq!(reallocate_v1(&mut [0, 2, 7, 0]), 5)
    // }

    // #[test]
    // fn cases_v2() {
    //     assert_eq!(reallocate_v2(&mut [0, 2, 7, 0]), 4)
    // }
}
