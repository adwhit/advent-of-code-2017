#![feature(inclusive_range_syntax)]

extern crate advent_of_code;
#[macro_use]
extern crate failure;

use advent_of_code::Result;

use std::fs;
use std::io::prelude::*;

fn get_data(path: &str) -> Result<Vec<Move>> {
    let mut f = fs::File::open(path)?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    s.split(',').map(|v| Move::parse(v.trim())).collect()
}

#[derive(Clone, Copy, Debug)]
enum Move {
    North,
    NorthEast,
    NorthWest,
    South,
    SouthEast,
    SouthWest,
}

impl Move {
    fn parse(v: &str) -> Result<Move> {
        use Move::*;
        Ok(match v {
            "n" => North,
            "nw" => NorthWest,
            "ne" => NorthEast,
            "s" => South,
            "sw" => SouthWest,
            "se" => SouthEast,
            _ => bail!("Bad string: {}", v),
        })
    }
}

#[derive(Default)]
struct Pos {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

impl Pos {
    fn mov(&mut self, m: Move) {
        use Move::*;
        match m {
            North => {
                self.y += 1;
                self.z -= 1
            }
            South => {
                self.y -= 1;
                self.z += 1;
            }
            NorthEast => {
                self.x += 1;
                self.z -= 1
            }
            SouthWest => {
                self.x -= 1;
                self.z += 1
            }
            NorthWest => {
                self.x -= 1;
                self.y += 1
            }
            SouthEast => {
                self.x += 1;
                self.y -= 1
            }
        }
    }

    fn distance(&self, p: &Pos) -> i32 {
        ((self.x - p.x).abs() + (self.y - p.y).abs() + (self.z - p.z).abs()) / 2
    }
}

fn hex_ed(moves: &[Move]) -> i32 {
    let mut pos = Pos::default();
    for mov in moves {
        pos.mov(*mov)
    }
    pos.distance(&Pos::default())
}

fn run() -> Result<()> {
    {
        let data = get_data("data/11.txt")?;
        let outcome = hex_ed(&data);
        println!("v1: {}", outcome);
    }

    // {
    //     let data = get_data2("data/10.txt")?;
    //     let outcome = knot_hash2(&mut state, &data);
    //     println!("v2: {}", outcome);
    // }
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
        use Move::*;
        let outcome = hex_ed(&[SouthEast, SouthWest, SouthEast, SouthWest, SouthWest]);
        assert_eq!(outcome, 3)
    }

    #[test]
    fn cases_v2() {}
}
