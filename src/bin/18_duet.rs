extern crate advent_of_code;
#[macro_use]
extern crate failure;
#[macro_use]
extern crate nom;

use advent_of_code::Result;

use std::fs;
use std::io::prelude::*;
use std::collections::HashMap;

use nom::{anychar, digit};

fn get_data(path: &str) -> Result<Vec<Instruction>> {
    let mut f = fs::File::open(path)?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    s.lines().map(|line| parse(line)
                  .to_result()
                  .map_err(|_| format_err!("Parse err: {}", line)))
        .collect()
}

named!(parse<&str, Instruction>, switch!(take!(3),
    "snd" => map!(anychar, |c| Instruction::Snd(c)) |
    "set" => map!(pair!(anychar, input), |(c, inp)| Instruction::Set(c, inp)) |
    "add" => map!(pair!(anychar, input), |(c, inp)| Instruction::Add(c, inp)) |
    "mul" => map!(pair!(anychar, input), |(c, inp)| Instruction::Mul(c, inp)) |
    "mod" => map!(pair!(anychar, input), |(c, inp)| Instruction::Mod(c, inp)) |
    "rcv" => map!(anychar, |c| Instruction::Rcv(c)) |
    "jgz" => map!(pair!(anychar, input), |(c, inp)| Instruction::Jgz(c, inp))
));

named!(input<&str, Input>, alt!(
    digit => { |ds: &str| Input::Val(ds.parse::<u32>().unwrap()) } |
    anychar => { |c| Input::Ptr(c) } ));

enum Input {
    Ptr(char),
    Val(u32)
}

enum Instruction {
    Snd(char),
    Set(char, Input),
    Add(char, Input),
    Mul(char, Input),
    Mod(char, Input),
    Rcv(char),
    Jgz(char, Input)
}

struct State {
    registers: HashMap<u8, u32>,
    code: Vec<Instruction>,
    codeptr: usize
}

impl State {
    fn new(code: Vec<Instruction>) -> State {
        State {
            registers: HashMap::new(),
            code,
            codeptr: 0
        }
    }
}

fn duet(code: Vec<Instruction>) -> usize {
    let state = State::new(code);
    10
}

fn run() -> Result<()> {
    {
        let data = get_data("data/18.txt")?;
        let outcome = duet(data);
        println!("v1: {}", outcome);
    }

    // {
    //     let data = get_data2("data/18.txt")?;
    //     let outcome = duet(&mut state, &data);
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
    }

    #[test]
    fn cases_v2() {
    }
}
