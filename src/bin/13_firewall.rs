extern crate advent_of_code;
#[macro_use]
extern crate failure;
#[macro_use]
extern crate nom;

use advent_of_code::Result;

use std::fs;
use std::io::prelude::*;

use nom::digit;

named!(levels<&str, (u32, u32)>, separated_pair!(
    map!(digit, |v| v.parse::<u32>().unwrap()),
    tag!(": "),
    map!(digit, |v| v.parse::<u32>().unwrap())));

fn get_data(path: &str) -> Result<Vec<(u32, u32)>> {
    let mut f = fs::File::open(path)?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    let out = s.lines()
        .map(|line| {
            levels(line)
                .to_result()
                .map_err(|_| format_err!("Parse fail: {:?}", line))
        })
        .collect::<Result<Vec<_>>>()?;
    Ok(out)
}

fn flatten(data: &[(u32, u32)]) -> Vec<Option<u32>> {
    let len = data.last().unwrap().0 + 1;
    let mut flat = vec![None; len as usize];
    for &(ix, val) in data {
        flat[ix as usize] = Some(val)
    }
    flat
}

fn firewall(data: &[(u32, u32)]) -> u32 {
    let flat = flatten(data);
    let mut severity = 0;
    for (ix, range) in flat.iter().enumerate() {
        let ix = ix as u32;
        if let &Some(range) = range {
            if ix % ((range - 1) * 2) == 0 {
                severity += range * ix
            }
        }
    }
    severity
}

fn run() -> Result<()> {
    let data = get_data("data/13.txt")?;
    let outcome = firewall(&data);
    println!("v1: {}", outcome);

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
    fn cases_v2() {
        let data = get_data("data/12_test.txt").unwrap();
        let outcome = plumber2(&data).unwrap();
        assert_eq!(outcome, 2);
    }
}
