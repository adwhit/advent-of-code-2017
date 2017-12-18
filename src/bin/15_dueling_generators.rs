#![feature(generators, generator_trait, conservative_impl_trait)]

use std::ops::{Generator, GeneratorState};

extern crate advent_of_code;
extern crate failure;
extern crate nom;

use advent_of_code::Result;

fn generatorer(start: i64, factor: i64) -> impl Generator<Yield = i64, Return = ()> {
    move || {
        let mut val = start;
        loop {
            val = (val * factor) % 2147483647;
            yield val
        }
    }
}

fn duel(starta: i64, startb: i64) -> u32 {
    let mut ct = 0;
    let mut gena = generatorer(starta, 16807);
    let mut genb = generatorer(startb, 48271);
    for _ in 0..40_000_000 {
        let a = match gena.resume() {
            GeneratorState::Yielded(y) => y,
            _ => unreachable!()
        };
        let b = match genb.resume() {
            GeneratorState::Yielded(y) => y,
            _ => unreachable!()
        };
        if a & 0xffff == b & 0xffff {
            ct += 1
        }
    }
    ct
}

fn run() -> Result<()> {
    let outcome = duel(873, 583);
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
        let outcome = duel(65, 8921);
        assert_eq!(outcome, 588);
    }

    // #[test]
    // fn cases_v2() {
    //     let data = get_data("data/12_test.txt").unwrap();
    //     let outcome = plumber2(&data).unwrap();
    //     assert_eq!(outcome, 2);
    // }
}
