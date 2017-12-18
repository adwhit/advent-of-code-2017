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
        .map(|v| {
            v.trim()
                .parse::<u8>()
                .map_err(|_| format_err!("parse fail: {:?}", v))
        })
        .collect()
}

fn get_data2(path: &str) -> Result<Vec<u8>> {
    let mut f = fs::File::open(path)?;
    let mut s = Vec::new();
    f.read_to_end(&mut s)?;
    s.pop(); // remove newline
    s.extend(&[17, 31, 73, 47, 23]);
    Ok(s)
}

#[derive(Clone, Debug)]
struct State {
    data: Vec<u8>,
    skip: usize,
    curpos: usize,
}

impl State {
    fn new(size: u8) -> State {
        State {
            data: (0..=size).collect(),
            skip: 0,
            curpos: 0,
        }
    }

    fn hash(&mut self, len: usize) {
        let l = self.data.len();
        for ix in 0..(len / 2) {
            self.data
                .swap((self.curpos + ix) % l, (self.curpos + len - ix - 1) % l);
        }
        self.curpos = (self.curpos + len + self.skip) % l;
        self.skip = (self.skip + 1) % l;
    }

    fn dense_hash(&self) -> String {
        let mut out = String::new();
        for round in 0..16 {
            let mut v = self.data[round * 16];
            for roundix in 1..16 {
                let ix = round * 16 + roundix;
                v ^= self.data[ix]
            }
            if v < 16 {
                out.push_str("0")
            }
            out.push_str(&format!("{:x}", v))
        }
        assert!(out.len() == 32);
        out
    }
}

fn knot_hash(state: &mut State, lens: &[u8]) -> u32 {
    for len in lens {
        state.hash(*len as usize)
    }
    state.data[0] as u32 * state.data[1] as u32
}

fn knot_hash2(state: &mut State, lens: &[u8]) -> String {
    for _ in 0..64 {
        for len in lens {
            state.hash(*len as usize)
        }
    }
    state.dense_hash()
}

fn run() -> Result<()> {
    {
        let data = get_data("data/10.txt")?;
        let mut state = State::new(255);
        let outcome = knot_hash(&mut state, &data);
        println!("v1: {}", outcome);
    }

    {
        let data = get_data2("data/10.txt")?;
        let mut state = State::new(255);
        let outcome = knot_hash2(&mut state, &data);
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
        let lens = &[3, 4, 1, 5];
        let mut state = State::new(4);
        let outcome = knot_hash(&mut state, lens);
        assert_eq!(outcome, 12);
    }

    #[test]
    fn cases_v2() {
        let mut lens = Vec::new();
        lens.extend(&[17, 31, 73, 47, 23]);
        let mut state = State::new(255);
        let outcome = knot_hash2(&mut state, &lens);
        let expect = "a2582a3a0e66e6e86e3812dcb672a272";
        assert_eq!(&outcome, expect);

        let mut lens = b"AoC 2017".to_vec();
        lens.extend(&[17, 31, 73, 47, 23]);
        let mut state = State::new(255);
        let outcome = knot_hash2(&mut state, &lens);
        let expect = "33efeb34ea91902bb2f59c9920caa6cd";
        assert_eq!(&outcome, expect);
    }
}
