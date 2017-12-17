#![feature(inclusive_range_syntax)]

extern crate advent_of_code;
#[macro_use]
extern crate failure;

use advent_of_code::Result;

use std::fs;
use std::io::prelude::*;

fn get_data(path: &str) -> Result<Vec<u8>> {
    let mut f = fs::File::open(path)?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    s.split(',')
        .map(
            |v| v.trim().parse::<u8>()
             .map_err(|_| format_err!("parse fail: {:?}", v))
        ).collect()
}

#[derive(Clone, Debug)]
struct State {
    data: Vec<u8>,
    skip: usize,
    curpos: usize
}

impl State {
    fn new(size: u8) -> State {
        State {
            data: (0..=size).collect(),
            skip: 0,
            curpos: 0
        }
    }

    fn hash(&mut self, len: usize) {
        let l = self.data.len();
        for ix in 0..(len / 2) {
            self.data.swap((self.curpos + ix) % l, (self.curpos + len - ix - 1) % l);
        }
        self.curpos = (self.curpos + len + self.skip) % l;
        self.skip = (self.skip + 1) % l;
    }
}

fn knot_hash(state: &mut State, lens: &[u8]) -> u32 {
    for len in lens {
        state.hash(*len as usize)
    }
    state.data[0] as u32 * state.data[1] as u32
}

fn run() -> Result<()> {
    let data = get_data("data/10.txt")?;
    let mut state = State::new(255);
    let outcome = knot_hash(&mut state, &data);
    println!("v1: {}", outcome);
    // println!("v2: {}", outcome1.1);
    // let outcome2 = circus_v2(&mut data);
    // println!("v2: {}", outcome2);
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
        let lens = &[3, 4, 1, 5];
        let mut state = State::new(4);
        let outcome = knot_hash(&mut state, lens);
        assert_eq!(outcome, 12);
    }
}
