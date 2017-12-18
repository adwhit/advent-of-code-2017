#![feature(generators, generator_trait, conservative_impl_trait, never_type)]

use std::ops::{Generator, GeneratorState};

extern crate advent_of_code;
extern crate failure;
extern crate nom;

use advent_of_code::Result;

fn generatorer(start: i64, factor: i64, mod_: i64) -> impl Generator<Yield = i64, Return = !> {
    move || {
        let mut val = start;
        loop {
            val = (val * factor) % 2147483647;
            if val % mod_ == 0 {
                yield val
            }
        }
    }
}

fn duel(starta: i64, moda: i64, startb: i64, modb: i64, max: i64) -> u32 {
    let mut ct = 0;
    let mut gena = generatorer(starta, 16807, moda);
    let mut genb = generatorer(startb, 48271, modb);
    for _ in 0..max {
        let GeneratorState::Yielded(a) = gena.resume();
        let GeneratorState::Yielded(b) = genb.resume();
        if a & 0xffff == b & 0xffff {
            ct += 1
        }
    }
    ct
}

fn run() -> Result<()> {
    let outcome = duel(873, 1, 583, 1, 40_000_000);
    println!("v1: {}", outcome);

    let outcome = duel(873, 4, 583, 8, 5_000_000);
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
        let outcome = duel(65, 1, 8921, 1, 40_000_000);
        assert_eq!(outcome, 588);
    }

    #[test]
    fn cases_v2() {
        let outcome = duel(65, 4, 8921, 8, 5_000_000);
        assert_eq!(outcome, 309);
    }
}
