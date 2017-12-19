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
use std::sync::mpsc;

use nom::{anychar, digit};

fn get_data(path: &str) -> Result<Vec<Instruction>> {
    let mut f = fs::File::open(path)?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    s.lines()
        .map(|line| {
            parse(line)
                .to_result()
                .map_err(|_| format_err!("Parse err: {}", line))
        })
        .collect()
}

named!(parse<&str, Instruction>, ws!(switch!(take!(3),
    "snd" => map!(ws!(input), |inp| Instruction::Snd(inp)) |
    "set" => map!(pair!(ws!(anychar), input), |(c, inp)| Instruction::Set(c, inp)) |
    "add" => map!(pair!(ws!(anychar), input), |(c, inp)| Instruction::Add(c, inp)) |
    "mul" => map!(pair!(ws!(anychar), input), |(c, inp)| Instruction::Mul(c, inp)) |
    "mod" => map!(pair!(ws!(anychar), input), |(c, inp)| Instruction::Mod(c, inp)) |
    "rcv" => map!(ws!(input), |inp| Instruction::Rcv(inp)) |
    "jgz" => map!(pair!(ws!(input), input), |(inp1, inp2)| Instruction::Jgz(inp1, inp2))
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
    Val(i64),
}

#[derive(Debug, Clone, Copy)]
enum Instruction {
    Snd(Input),
    Set(char, Input),
    Add(char, Input),
    Mul(char, Input),
    Mod(char, Input),
    Rcv(Input),
    Jgz(Input, Input),
}

#[derive(Debug)]
struct State {
    pid: i64,
    registers: HashMap<char, i64>,
    code: Vec<Instruction>,
    codeptr: i64,
    last_freq: i64,
    tx: mpsc::Sender<i64>,
    rx: mpsc::Receiver<i64>,
    sendct: i64,
}

impl State {
    fn new(code: Vec<Instruction>) -> State {
        let (tx, rx) = mpsc::channel();
        State {
            pid: 0,
            registers: HashMap::new(),
            code,
            codeptr: 0,
            last_freq: 0,
            tx,
            rx,
            sendct: 0,
        }
    }

    fn new2(
        code: Vec<Instruction>,
        pid: i64,
        tx: mpsc::Sender<i64>,
        rx: mpsc::Receiver<i64>,
    ) -> State {
        let mut registers = HashMap::new();
        registers.insert('p', pid);
        State {
            pid,
            registers,
            code,
            codeptr: 0,
            last_freq: 0,
            tx,
            rx,
            sendct: 0,
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
                }
                Add(c, inp) => {
                    let v = self.input2val(inp);
                    let r = self.registers.entry(c).or_insert(0);
                    *r += v
                }
                Mul(c, inp) => {
                    let v = self.input2val(inp);
                    let r = self.registers.entry(c).or_insert(0);
                    *r *= v
                }
                Mod(c, inp) => {
                    let v = self.input2val(inp);
                    let r = self.registers.entry(c).or_insert(0);
                    *r %= v
                }
                Jgz(inp1, inp2) => {
                    let v1 = self.input2val(inp1);
                    if v1 > 0 {
                        let v2 = self.input2val(inp2);
                        self.codeptr += v2;
                        continue;
                    }
                }
                Snd(inp) => {
                    let v = self.input2val(inp);
                    self.last_freq = v;
                }
                Rcv(inp) => {
                    let v = self.input2val(inp);
                    if v > 0 {
                        return self.last_freq;
                    }
                }
            }
            self.codeptr += 1;
            if self.codeptr >= self.code.len() as i64 {
                panic!()
            }
        }
    }

    fn run2(&mut self) -> i64 {
        use Instruction::*;
        loop {
            let inst = self.code[self.codeptr as usize];
            match inst {
                Set(c, inp) => {
                    let v = self.input2val(inp);
                    let r = self.registers.entry(c).or_insert(0);
                    *r = v
                }
                Add(c, inp) => {
                    let v = self.input2val(inp);
                    let r = self.registers.entry(c).or_insert(0);
                    *r += v
                }
                Mul(c, inp) => {
                    let v = self.input2val(inp);
                    let r = self.registers.entry(c).or_insert(0);
                    *r *= v
                }
                Mod(c, inp) => {
                    let v = self.input2val(inp);
                    let r = self.registers.entry(c).or_insert(0);
                    *r %= v
                }
                Jgz(inp1, inp2) => {
                    let v1 = self.input2val(inp1);
                    if v1 > 0 {
                        let v2 = self.input2val(inp2);
                        self.codeptr += v2;
                        continue;
                    }
                }
                Snd(inp) => {
                    let v = self.input2val(inp);
                    self.tx.send(v).unwrap();
                    self.sendct += 1;
                }
                Rcv(inp) => {
                    if let Input::Ptr(c) = inp {
                        let r = self.registers.entry(c).or_insert(0);
                        if self.pid == 0 {
                            match self.rx.recv() {
                                Ok(v) => *r = v,
                                Err(_) => return 0, // finished
                            }
                        } else {
                            let duration = std::time::Duration::from_millis(10);
                            match self.rx.recv_timeout(duration) {
                                Ok(v) => *r = v,
                                Err(mpsc::RecvTimeoutError::Timeout) => return self.sendct,
                                _ => panic!("Disconnected!"),
                            }
                        }
                    } else {
                        panic!()
                    }
                }
            }
            self.codeptr += 1;
            if self.codeptr >= self.code.len() as i64 {
                return self.sendct;
            }
        }
    }

    fn input2val(&mut self, inp: Input) -> i64 {
        match inp {
            Input::Ptr(reg) => {
                let v = self.registers.entry(reg).or_default();
                *v
            }
            Input::Val(v) => v,
        }
    }
}

fn duet(code: Vec<Instruction>) -> i64 {
    let mut state = State::new(code);
    state.run()
}

fn duet2(code: Vec<Instruction>) -> i64 {
    let (tx0, rx1) = mpsc::channel();
    let (tx1, rx0) = mpsc::channel();
    let mut state0 = State::new2(code.clone(), 0, tx0, rx0);
    let mut state1 = State::new2(code, 1, tx1, rx1);
    std::thread::spawn(move || state0.run2());
    state1.run2()
}

fn run() -> Result<()> {
    {
        let data = get_data("data/18.txt")?;
        let outcome = duet(data);
        println!("v1: {}", outcome);
    }

    {
        let data = get_data("data/18.txt")?;
        let outcome = duet2(data);
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
        let data = get_data("data/18_test.txt").unwrap();
        let outcome = duet(data);
        assert_eq!(outcome, 4)
    }

    #[test]
    fn cases_v2() {
        let data = get_data("data/18_test.txt").unwrap();
        let outcome = duet2(data);
        assert_eq!(outcome, 1);

        let data = get_data("data/18_test2.txt").unwrap();
        let outcome = duet2(data);
        assert_eq!(outcome, 3);
    }
}
