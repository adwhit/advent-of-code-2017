extern crate advent_of_code;
extern crate failure;
extern crate itertools;

use advent_of_code::Result;

use std::fs;
use std::io::prelude::*;
use itertools::{Itertools};

fn get_data() -> Result<Vec<Vec<String>>> {
    let mut f = fs::File::open("data/04.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s.lines().map(|line| {
        line.split_whitespace().map(String::from).collect()
    }).collect())
}

fn contains_anagrams(words: &[&str]) -> bool {
    let ct = words.iter().map(|word| word.chars().sorted()).sorted().iter().dedup().count();
    ct != words.len()
}

fn passphrase_v1(rows: &[&[&str]]) -> u32 {
    rows.iter().filter(|row| {
        let unqs = row.to_vec().iter().sorted().iter().dedup().count();
        unqs == row.len()
    }).count() as u32
}

fn passphrase_v2(rows: &[&[&str]]) -> u32 {
    rows.iter().filter(|row| {
        !contains_anagrams(row)
    }).count() as u32
}

fn run() -> Result<()> {
    let data = get_data()?;
    let data: Vec<Vec<&str>> = data.iter().map(|row| row.iter().map(|s| s.as_ref()).collect()).collect_vec();
    let data: Vec<&[&str]> = data.iter().map(
        |row| row.as_slice()
    ).collect();
    let outcome1 = passphrase_v1(&data);
    println!("v1: {}", outcome1);
    let outcome2 = passphrase_v2(&data);
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
        assert!(contains_anagrams(&["abcde", "edcba"]));
        assert!(!contains_anagrams(&["abcde", "ccccc", "ccddd"]));
        assert!(!contains_anagrams(&["abde", "edcba", "ccc"]));
    }
}
