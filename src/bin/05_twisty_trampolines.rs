extern crate advent_of_code;
#[macro_use]
extern crate failure;

use advent_of_code::Result;

use std::fs;
use std::io::prelude::*;

fn get_data() -> Result<Vec<i32>> {
    let mut f = fs::File::open("data/05.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    s.lines()
        .map(|line| {
            line.parse::<i32>()
                .map_err(|e| format_err!("Failed to parse: {}", e))
        })
        .collect()
}

fn trampoline_v1(data: &mut [i32]) -> u32 {
    let mut loc = 0i32;
    let mut steps = 0;
    loop {
        if loc < 0 || loc as usize >= data.len() {
            return steps;
        }
        let diff = data[loc as usize];
        data[loc as usize] += 1;
        loc += diff;
        steps += 1;
    }
}

fn trampoline_v2(data: &mut [i32]) -> u32 {
    let mut loc = 0i32;
    let mut steps = 0;
    loop {
        if loc < 0 || loc as usize >= data.len() {
            return steps;
        }
        let diff = data[loc as usize];
        if diff >= 3 {
            data[loc as usize] -= 1;
        } else {
            data[loc as usize] += 1;
        }
        loc += diff;
        steps += 1;
    }
}

fn run() -> Result<()> {
    let mut data = get_data()?;
    let outcome1 = trampoline_v1(&mut data);
    println!("v1: {}", outcome1);
    let mut data = get_data()?;
    let outcome2 = trampoline_v2(&mut data);
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
        assert_eq!(trampoline_v1(&mut [0, 3, 0, 1, -3]), 5)
    }

    #[test]
    fn cases_v2() {
        assert_eq!(trampoline_v2(&mut [0, 3, 0, 1, -3]), 10)
    }
}
