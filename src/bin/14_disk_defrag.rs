#![feature(inclusive_range_syntax)]

extern crate advent_of_code;
extern crate failure;

use advent_of_code::Result;
use std::collections::{HashMap};

type Array = [[bool; 128]; 128];

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
    let array = used_array(key);
    let mut bittot = 0;
    for rowix in 0..128 {
        for colix in 0..128 {
            if array[rowix][colix] {
                bittot += 1
            }
        }
    }
    bittot
}

fn used_array(key: &[u8]) -> Array {
    let mut array = [[false; 128]; 128];
    for rowix in 0..128 {
        let mut inner = [false; 128];
        let mut key = key.to_vec();
        key.extend_from_slice((&format!("-{}", rowix).as_bytes()));
        let hash = knot_hash(key);
        for (ix, byte) in hash.iter().enumerate() {
            for bit in 0..8u8 {
                let in_use = if ((byte >> (7 - bit)) & 1u8) == 1 { true } else { false };
                inner[ix * 8 + bit as usize] = in_use
            }
        }
        array[rowix] = inner
    }
    array
}


fn connected_components(array: &Array) -> u32 {
    let mut grpct = 0;
    let mut seen = HashMap::new();
    for rowix in 0..128 {
        for colix in 0..128 {
            if array[rowix][colix] {
                if seen.get(&(rowix, colix)).is_none() {
                    // We have found a new group, do a depth-first search
                    seen.insert((rowix, colix), grpct);
                    dfs(rowix, colix, array, &mut seen, grpct);
                    grpct += 1;
                }
            }
        }
    }
    grpct
}

fn dfs(rowix: usize, colix: usize, array: &Array, seen: &mut HashMap<(usize, usize), u32>, grpct: u32) {
    for &(r, c) in &[(-1, 0), (1, 0), (0, -1), (0, 1)] {
        let rix = (rowix as isize + r) as usize;
        let cix = (colix as isize + c) as usize;
        if *array.get(rix).and_then(|arr| arr.get(cix)).unwrap_or(&false) {
            if seen.get(&(rix, cix)).is_none() {
                seen.insert((rix, cix), grpct);
                dfs(rix, cix, array, seen, grpct)
            }
        }
    }
}

fn defrag2(key: &[u8]) -> u32 {
    let array = used_array(key);
    connected_components(&array)
}

fn run() -> Result<()> {
    let data = b"hfdlxzhv";
    let outcome = defrag(data);
    println!("v1: {}", outcome);

    let outcome = defrag2(data);
    println!("v2: {}", outcome);

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
    fn cases_v2() {
        let data = b"flqrgnkx";
        let outcome = defrag2(data);
        assert_eq!(outcome, 1242);
    }
}
