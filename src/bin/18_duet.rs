#![feature(entry_or_default)]

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

named!(parse<&str, Instruction>, ws!(switch!(take!(3),
    "snd" => map!(ws!(input), |inp| Instruction::Snd(inp)) |
    "set" => map!(pair!(ws!(anychar), input), |(c, inp)| Instruction::Set(c, inp)) |
    "add" => map!(pair!(ws!(anychar), input), |(c, inp)| Instruction::Add(c, inp)) |
    "mul" => map!(pair!(ws!(anychar), input), |(c, inp)| Instruction::Mul(c, inp)) |
    "mod" => map!(pair!(ws!(anychar), input), |(c, inp)| Instruction::Mod(c, inp)) |
    "rcv" => map!(ws!(input), |inp| Instruction::Rcv(inp)) |
    "jgz" => map!(pair!(ws!(anychar), input), |(c, inp)| Instruction::Jgz(c, inp))
)));

named!(input<&str, Input>, alt!(
    pair!(opt!(tag!("-")), digit) => { |(n, ds): (Option<&str>, &str)| {
        let neg = n.map_or(1, |_| -1);
        Input::Val(neg * ds.parse::<i64>().unwrap())
    }} |
    anychar => { |c| Input::Ptr(c) } ));

#[derive(Debug, Clone, Copy)]
enum Input {
    Ptr(char),
    Val(i64)
}

#[derive(Debug, Clone, Copy)]
enum Instruction {
    Snd(Input),
    Set(char, Input),
    Add(char, Input),
    Mul(char, Input),
    Mod(char, Input),
    Rcv(Input),
    Jgz(char, Input)
}

#[derive(Debug, Clone)]
struct State {
    registers: HashMap<char, i64>,
    code: Vec<Instruction>,
    codeptr: i64,
    last_freq: i64
}

impl State {
    fn new(code: Vec<Instruction>) -> State {
        State {
            registers: HashMap::new(),
            code,
            codeptr: 0,
            last_freq: 0
        }
    }

    fn run(&mut self) -> i64 {
        use Instruction::*;
        loop {
            let inst = self.code[self.codeptr as usize];
            match inst {
                Set(c, inp) => {
                    let v = self.input2val(inp);
                    let r = self.registers.entry(c).or_insert(0);
                    *r = v
                },
                Add(c, inp) => {
                    let v = self.input2val(inp);
                    let r = self.registers.entry(c).or_insert(0);
                    *r += v
                },
                Mul(c, inp) => {
                    let v = self.input2val(inp);
                    let r = self.registers.entry(c).or_insert(0);
                    *r *= v
                },
                Mod(c, inp) => {
                    let v = self.input2val(inp);
                    let r = self.registers.entry(c).or_insert(0);
                    *r %= v
                },
                Jgz(c, inp) => {
                    let r = *self.registers.entry(c).or_insert(0);
                    if r > 0 {
                        let v = self.input2val(inp);
                        self.codeptr += v;
                        continue
                    }
                },
                Snd(inp) => {
                    let v = self.input2val(inp);
                    self.last_freq = v;
                },
                Rcv(inp) => {
                    let v = self.input2val(inp);
                    if v > 0 {
                        return self.last_freq
                    }
                }
            }
            self.codeptr += 1;
            if self.codeptr >= self.code.len() as i64 {
                panic!()
            }
        }
    }

    fn input2val(&mut self, inp: Input) -> i64 {
        match inp {
            Input::Ptr(reg) => {
                let v = self.registers.entry(reg).or_default();
                *v
            }
            Input::Val(v) => v
        }
    }
}

fn duet(code: Vec<Instruction>) -> i64 {
    let mut state = State::new(code);
    state.run()
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
        let data = get_data("data/18_test.txt").unwrap();
        let outcome = duet(data);
        assert_eq!(outcome, 4)
    }

    // #[test]
    // fn cases_v2() {
    // }
}
