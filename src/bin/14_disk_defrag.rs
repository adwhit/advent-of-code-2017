#![feature(inclusive_range_syntax)]

extern crate advent_of_code;
extern crate failure;

use advent_of_code::Result;

#[derive(Clone, Debug)]
struct State {
    data: Vec<u8>,
    skip: usize,
    curpos: usize,
}

impl State {
    fn new() -> State {
        State {
            data: (0..=255).collect(),
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

    fn dense_hash(&self) -> [u8; 16] {
        let mut out = [0; 16];
        for round in 0..16 {
            let mut v = self.data[round * 16];
            for roundix in 1..16 {
                let ix = round * 16 + roundix;
                v ^= self.data[ix]
            }
            out[round] = v
        }
        out
    }
}

fn knot_hash(mut key: Vec<u8>) -> [u8; 16] {
    key.extend(&[17, 31, 73, 47, 23]);
    let mut state = State::new();
    for _ in 0..64 {
        for k in &key {
            state.hash(*k as usize)
        }
    }
    state.dense_hash()
}

fn defrag(key: &[u8]) -> u32 {
    let mut bittot = 0;
    for rowix in 0..128 {
        let mut key = key.to_vec();
        key.extend_from_slice((&format!("-{}", rowix).as_bytes()));
        let hash = knot_hash(key);
        for byte in &hash {
            for bit in 0..8 {
                let bitv = ((byte >> (7 - bit)) & 1u8) as u32;
                bittot += bitv;
            }
        }
    }
    bittot
}

fn run() -> Result<()> {
    let data = b"hfdlxzhv";
    let outcome = defrag(data);
    println!("v1: {}", outcome);

    // let outcome = plumber2(&data)?;
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
        let data = b"flqrgnkx";
        let outcome = defrag(data);
        assert_eq!(outcome, 8108);
    }

    #[test]
    fn cases_v2() {}
}
