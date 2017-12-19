#![feature(inclusive_range_syntax)]

extern crate advent_of_code;
#[macro_use]
extern crate failure;

use advent_of_code::Result;
use std::collections::LinkedList;

fn spinlock(step: usize) -> u32 {
    let mut list = LinkedList::new();
    list.push_back(0);
    let mut curpos = 0;
    for val in 1..2018 {
        curpos = (curpos + step) % list.len();
        let mut rest = list.split_off(curpos + 1);
        list.push_back(val);
        list.append(&mut rest);
        curpos += 1;
    }
    let respos = (curpos + 1) % list.len();
    *list.iter().nth(respos).unwrap()
}

fn run() -> Result<()> {
    {
        let outcome = spinlock(316);
        println!("v1: {}", outcome);
    }

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
        let next = spinlock(3);
        assert_eq!(next, 638);
    }

    #[test]
    fn cases_v2() {}
}
