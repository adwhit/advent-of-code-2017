extern crate advent_of_code;
#[macro_use]
extern crate failure;
extern crate itertools;
#[macro_use]
extern crate nom;

use advent_of_code::Result;
use nom::{alpha, digit};

use std::fs;
use std::result::Result as StdResult;
use std::io::prelude::*;
use std::collections::HashMap;

fn get_data(path: &str) -> Result<Vec<String>> {
    let mut f = fs::File::open(path)?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s.lines().map(String::from).collect())
}

#[derive(Clone, Debug)]
struct Instruction {
    reg: String,
    op: Op,
    val: i32,
    pred_reg: String,
    test_op: TestOp,
    test_val: i32
}

#[derive(Clone, Copy, Debug)]
enum Op {
    Inc,
    Dec
}

#[derive(Clone, Copy, Debug)]
enum TestOp {
    Gt,
    Gte,
    Eq,
    Neq,
    Lte,
    Lt
}


named!(parse_instruction<&str, Instruction>, do_parse!(
    reg: map!(alpha, String::from) >> tag!(" ") >>
    op: alt!(
        tag_s!("inc") => { |_| Op::Inc } |
        tag_s!("dec") => { |_| Op::Dec }
        ) >> tag!(" ") >>
   neg: map!(opt!(tag!("-")), |v| v.map_or(1, |_| -1)) >>
   val: map!(digit, |v| v.parse::<i32>().unwrap() * neg) >> tag!(" if ") >>
   pred_reg: map!(alpha, String::from) >> tag!(" ") >>
   test_op: alt!(
       tag_s!(">=") => { |_| TestOp::Gte } |
       tag_s!("<=") => { |_| TestOp::Lte } |
       tag_s!("==") => { |_| TestOp::Eq } |
       tag_s!("!=") => { |_| TestOp::Neq } |
       tag_s!(">") => { |_| TestOp::Gt } |
       tag_s!("<") => { |_| TestOp::Lt }
   ) >> tag!(" ") >>
   tneg: map!(opt!(tag!("-")), |v| v.map_or(1, |_| -1)) >>
   test_val: map!(digit, |v| v.parse::<i32>().unwrap() * tneg) >>
   ( Instruction { reg, op, val, pred_reg, test_op, test_val })
));

fn interpret(instructions: &[Instruction]) -> Result<HashMap<&str, i32>> {
    let mut state: HashMap<&str, i32> = instructions.iter().map(|i| (i.reg.as_str(), 0)).collect();
    for i in instructions {
        let pred = {
            use TestOp::*;
            let v = *(state.get(i.pred_reg.as_str()).ok_or(format_err!("Key not found"))?);
            let tv = i.test_val;
            match i.test_op {
                Gt => v > tv,
                Gte => v >= tv,
                Eq => v == tv,
                Neq => v != tv,
                Lte => v <= tv,
                Lt => v < tv
            }
        };
        if pred {
            let v = state.get_mut(i.reg.as_str()).ok_or(format_err!("Key not found"))?;
            match i.op {
                Op::Inc => *v += i.val,
                Op::Dec => *v -= i.val
            }
        }
    }
    Ok(state)
}

fn registers_v1(lines: &[String]) -> Result<i32> {
    let instructions = lines
        .iter()
        .map(|i| parse_instruction(&i).to_result())
        .collect::<StdResult<Vec<_>, _>>()?;
    let state = interpret(&instructions)?;
    Ok(*state.values().max().unwrap())
}

fn run() -> Result<()> {
    let lines = get_data("data/08.txt")?;
    let outcome1 = registers_v1(&lines)?;
    println!("v1: {}", outcome1);
    // let mut data = get_data("data/07.txt")?;
    // let outcome2 = circus_v2(&mut data);
    // println!("v2: {}", outcome2);
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
        let mut data = get_data("data/08_test.txt").unwrap();
        let outcome = registers_v1(&mut data).unwrap();
        assert_eq!(outcome, 1)
    }

    // #[test]
    // fn cases_v2() {
    //     let mut data = get_data("data/08_test.txt").unwrap();
    //     let outcome2 = circus_v2(&mut data);
    //     assert_eq!(outcome2, 243)
    // }
}
