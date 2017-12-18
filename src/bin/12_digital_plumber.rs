#![feature(inclusive_range_syntax)]

extern crate advent_of_code;
#[macro_use]
extern crate failure;
#[macro_use]
extern crate nom;

use advent_of_code::Result;

use std::fs;
use std::result::Result as StdResult;
use std::collections::{HashMap, HashSet};
use std::io::prelude::*;

use nom::digit;

type Map = HashMap<i32, Vec<i32>>;

named!(routes<&str, (i32, Vec<i32>)>, separated_pair!(
    map!(digit, |v| v.parse::<i32>().unwrap()),
    tag!(" <-> "),
    separated_nonempty_list_complete!(tag!(", "),
                             map!(digit, |v| v.parse::<i32>().unwrap()))));

fn get_data(path: &str) -> Result<Map> {
    let mut f = fs::File::open(path)?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    let r = s.lines()
        .map(|line| routes(line).to_result())
        .collect::<StdResult<Vec<_>, _>>()?;
    Ok(r.into_iter().collect())
}

fn plumber_(lookup: i32, data: &Map, seen: &mut HashSet<i32>) -> Result<()> {
    let links = data.get(&lookup).ok_or_else(|| format_err!("Not found: {}", lookup))?;
    for link in links {
        if seen.insert(*link) {
            plumber_(*link, data, seen)?
        }
    }
    Ok(())
}

fn plumber(data: &Map) -> Result<i32> {
    let mut seen = HashSet::new();
    plumber_(0, data, &mut seen)?;
    Ok(seen.len() as i32)
}

fn run() -> Result<()> {
    let data = get_data("data/12.txt")?;
    let outcome = plumber(&data)?;
    println!("v1: {}", outcome);

    // let outcome = hex_ed2(&data);
    // println!("v2: {}", outcome);
    Ok(())
}

fn main() {
    run().unwrap_or_else(|e| {
        println!("Error: {}", e);
        for cause in e.causes() {
            println!("{}", cause)
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cases_v1() {
        let data = get_data("data/12_test.txt").unwrap();
        let outcome = plumber(&data).unwrap();
        assert_eq!(outcome, 6);
    }

    #[test]
    fn cases_v2() {}
}
