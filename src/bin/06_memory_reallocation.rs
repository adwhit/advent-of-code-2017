extern crate advent_of_code;
#[macro_use]
extern crate failure;

use advent_of_code::Result;

use std::fs;
use std::io::prelude::*;
use std::collections::{HashMap, HashSet};

fn get_data() -> Result<Vec<u32>> {
    let mut f = fs::File::open("data/06.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    s.split_whitespace()
        .map(|val| {
            val.parse::<u32>()
                .map_err(|e| format_err!("Failed to parse: {}", e))
        })
        .collect()
}

fn get_max_mix(data: &[u32]) -> (u32, usize) {
    data.iter().enumerate().fold(
        (0, 0),
        |(max, mix), (ix, &val)| {
            if val > max {
                (val, ix)
            } else {
                (max, mix)
            }
        },
    )
}

fn reallocate_v1(data: &mut [u32]) -> u32 {
    let mut set = HashSet::new();
    for i in 0.. {
        if !set.insert(data.to_vec()) {
            return i;
        }
        let (max, mix) = get_max_mix(data);
        let mut remains = max as usize;
        data[mix] = 0;
        remains -= data.iter_mut()
            .skip(mix + 1)
            .take(remains)
            .map(|v| *v += 1)
            .count();
        while remains > 0 {
            remains -= data.iter_mut().take(remains).map(|v| *v += 1).count();
        }
    }
    unreachable!()
}

fn reallocate_v2(data: &mut [u32]) -> u32 {
    let mut map = HashMap::new();
    for _ in 0.. {
        if let Some(v) = map.insert(data.to_vec(), 0) {
            return v;
        }
        for v in map.values_mut() {
            *v += 1
        }
        let (max, mix) = get_max_mix(data);
        let mut remains = max as usize;
        data[mix] = 0;
        remains -= data.iter_mut()
            .skip(mix + 1)
            .take(remains)
            .map(|v| *v += 1)
            .count();
        while remains > 0 {
            remains -= data.iter_mut().take(remains).map(|v| *v += 1).count();
        }
    }
    unreachable!()
}

fn run() -> Result<()> {
    let mut data = get_data()?;
    let outcome1 = reallocate_v1(&mut data);
    println!("v1: {}", outcome1);
    let mut data = get_data()?;
    let outcome2 = reallocate_v2(&mut data);
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
        assert_eq!(reallocate_v1(&mut [0, 2, 7, 0]), 5)
    }

    #[test]
    fn cases_v2() {
        assert_eq!(reallocate_v2(&mut [0, 2, 7, 0]), 4)
    }
}
