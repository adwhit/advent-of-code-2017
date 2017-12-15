extern crate advent_of_code;
#[macro_use]
extern crate failure;
extern crate itertools;

use advent_of_code::Result;

use std::fs;
use std::io::prelude::*;
use itertools::{Itertools, MinMaxResult};

fn get_data() -> Result<Vec<Vec<u32>>> {
    let mut f = fs::File::open("data/02.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    s.lines().map(|line| {
        line.split_whitespace().map(|val| {
            val.parse::<u32>().map_err(|e| format_err!("Parse error: {}", e))
        }).collect()
    }).collect()
}

fn checksum_v1(rows: &[&[u32]]) -> u32 {
    rows.iter().map(|&row| {
        if let MinMaxResult::MinMax(min, max) = row.iter().minmax() {
            *max - *min
        } else {
            panic!("No min-max")
        }
    }).sum()
}

fn checksum_v2(rows: &[&[u32]]) -> u32 {
    rows.iter().map(|&row| {
        for v1 in row {
            for v2 in row {
                if v1 > v2 && v1 % v2 == 0 {
                    return v1 / v2
                }
            }
        }
        panic!("No evenly divisibles")
    }).sum()
}

fn run() -> Result<()> {
    let data = get_data()?;
    let data: Vec<&[u32]> = data.iter().map(|row| row.as_slice()).collect();
    let outcome1 = checksum_v1(&data);
    println!("v1: {}", outcome1);
    let outcome2 = checksum_v2(&data);
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
    fn cases_v1() {
        assert_eq!(checksum_v1(&[
            &[5, 1, 9, 5],
            &[7, 5, 3],
            &[2, 4, 6, 8]
      ]), 18)
    }

    #[test]
    fn cases_v2() {
        assert_eq!(checksum_v2(&[
            &[5, 9, 2, 8],
            &[9, 4, 7, 3],
            &[3, 8, 6, 5]
        ]), 9)
    }
}
