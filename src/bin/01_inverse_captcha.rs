extern crate advent_of_code;
#[macro_use]
extern crate failure;

use advent_of_code::Result;

use std::fs;
use std::io::prelude::*;

fn get_data() -> Result<Vec<u8>> {
    let mut f = fs::File::open("data/01.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    s.chars()
        .filter(|&c| c != '\n')
        .map(|c| {
            match c {
                '0'...'9' => Ok(c as u8 - '0' as u8),
                _ => bail!("Expected int, found {:?}", c)
            }
        }).collect()
}

fn captcha(mut data: Vec<u8>) -> u32 {
    let x = data[0];
    data.push(x);
    data.iter().zip(data.iter().skip(1)).fold(0, |acc, (v, vnext)| {
        if v == vnext {
            acc + *v as u32
        } else {
            acc
        }
    })
}

fn run() -> Result<()> {
    let data = get_data()?;
    let outcome = captcha(data);
    println!("{}", outcome);
    Ok(())
}

fn main() {
    run().unwrap_or_else(|e| {
        for cause in e.causes() {
            println!("{}", cause)
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cases() {
        assert_eq!(captcha(vec![1, 1, 2, 2]), 3);
        assert_eq!(captcha(vec![1, 1, 1, 1]), 4);
        assert_eq!(captcha(vec![1, 2, 3, 4]), 0);
        assert_eq!(captcha(vec![9, 1, 2, 1, 2, 1, 2, 9]), 9);
    }
}
