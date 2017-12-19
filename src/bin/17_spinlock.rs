#![feature(inclusive_range_syntax)]

extern crate advent_of_code;
extern crate failure;

use advent_of_code::Result;
use std::collections::LinkedList;

fn spinlock(step: usize, ct: u32) -> u32 {
    let mut list = LinkedList::new();
    list.push_back(0);
    let mut curpos = 0;
    for val in 1..ct + 1 {
        curpos = (curpos + step) % list.len();
        let mut rest = list.split_off(curpos + 1);
        list.push_back(val);
        list.append(&mut rest);
        curpos += 1;
    }
    let respos = (curpos + 1) % list.len();
    *list.iter().nth(respos).unwrap()
}

fn spinlock2(step: u32, ct: u32) -> u32 {
    let mut llen = 1;
    let mut curpos = 0;
    let mut zero_follow = 0;
    for val in 1..ct {
        curpos = (curpos + step) % llen;
        if curpos == 0 {
            zero_follow = val
        }
        curpos += 1;
        llen += 1;
    }
    zero_follow
}

fn run() -> Result<()> {
    {
        let outcome = spinlock(316, 2017);
        println!("v1: {}", outcome);
    }
    {
        let outcome = spinlock2(316, 50_000_000);
        println!("v2: {}", outcome);
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
        let next = spinlock(3, 2017);
        assert_eq!(next, 638);
    }

    #[test]
    fn cases_v2() {
        let next = spinlock2(3, 2017);
        assert_eq!(next, 1226);
    }
}
