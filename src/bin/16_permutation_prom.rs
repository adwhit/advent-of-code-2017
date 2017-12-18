#![feature(slice_rotate)]

extern crate advent_of_code;
#[macro_use]
extern crate failure;
#[macro_use]
extern crate nom;

use advent_of_code::Result;

use std::fs;
use std::io::prelude::*;
use std::str::from_utf8;

use nom::digit;

named!(
    parse<Motion>,
    switch!(take!(1),
    b"x" => map!(separated_pair!(
        map!(digit, |v| from_utf8(v).unwrap().parse::<usize>().unwrap()),
        tag!("/"),
        map!(digit, |v| from_utf8(v).unwrap().parse::<usize>().unwrap())),
        |(x, y)| Motion::Exchange(x, y)) |
    b"s" => map!(digit, |v| Motion::Spin(from_utf8(v).unwrap().parse::<usize>().unwrap())) |
    b"p" => map!(separated_pair!(
        take!(1),
        tag!("/"),
        take!(1)),
        |(a, b)| Motion::Partner(a[0], b[0])))
);

#[derive(Debug, Clone)]
enum Motion {
    Exchange(usize, usize),
    Spin(usize),
    Partner(u8, u8),
}

fn dance(motions: &[Motion]) -> Vec<u8> {
    use Motion::*;
    let mut seq: Vec<u8> = b"abcdefghijklmnop".to_vec();
    for motion in motions {
        match *motion {
            Exchange(ix1, ix2) => seq.swap(ix1, ix2),
            Spin(amt) => seq.rotate(16 - amt),
            Partner(n1, n2) => {
                let ix1 = seq.iter().position(|&v| v == n1).unwrap();
                let ix2 = seq.iter().position(|&v| v == n2).unwrap();
                seq.swap(ix1, ix2)
            }
        }
    }
    seq
}

fn dance2(motions: &[Motion]) -> Vec<u8> {
    // detect period
    use Motion::*;
    let init = b"abcdefghijklmnop";
    let mut period = 0;
    {
        let mut seq: Vec<u8> = init.to_vec();
        let cycle = motions.iter().cycle().enumerate();
        for (ix, motion) in cycle {
            match *motion {
                Exchange(ix1, ix2) => seq.swap(ix1, ix2),
                Spin(amt) => seq.rotate(16 - amt),
                Partner(n1, n2) => {
                    let ix1 = seq.iter().position(|&v| v == n1).unwrap();
                    let ix2 = seq.iter().position(|&v| v == n2).unwrap();
                    seq.swap(ix1, ix2)
                }
            }
            if &init[..] == &seq[..] {
                period = (ix + 1) / motions.len();
                break;
            }
        }
    }
    let mut seq: Vec<u8> = init.to_vec();
    for _ in 0..1_000_000_000 % period {
        for motion in motions {
            match *motion {
                Exchange(ix1, ix2) => seq.swap(ix1, ix2),
                Spin(amt) => seq.rotate(16 - amt),
                Partner(n1, n2) => {
                    let ix1 = seq.iter().position(|&v| v == n1).unwrap();
                    let ix2 = seq.iter().position(|&v| v == n2).unwrap();
                    seq.swap(ix1, ix2)
                }
            }
        }
    }
    seq
}

fn get_data(path: &str) -> Result<Vec<Motion>> {
    let mut f = fs::File::open(path)?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    s.split(',')
        .map(|v| {
            parse(v.trim().as_bytes())
                .to_result()
                .map_err(|_| format_err!("parse fail: {:?}", v))
        })
        .collect()
}

fn run() -> Result<()> {
    let data = get_data("data/16.txt")?;

    {
        let outcome = dance(&data);
        let outcome = String::from_utf8_lossy(&outcome);
        println!("v1: {}", outcome);
    }

    {
        let outcome = dance2(&data);
        let outcome = String::from_utf8_lossy(&outcome);
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
