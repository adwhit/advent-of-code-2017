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
        .map(|c| match c {
            '0'...'9' => Ok(c as u8 - '0' as u8),
            _ => bail!("Expected int, found {:?}", c),
        })
        .collect()
}

fn captcha(data: &[u8], skip: usize) -> u32 {
    data.iter()
        .zip(data.iter().cycle().skip(skip))
        .fold(
            0,
            |acc, (v, vnext)| {
                if v == vnext {
                    acc + *v as u32
                } else {
                    acc
                }
            },
        )
}

fn captcha_v1(data: &[u8]) -> u32 {
    captcha(data, 1)
}

fn captcha_v2(data: &[u8]) -> u32 {
    captcha(data, data.len() / 2)
}

fn run() -> Result<()> {
    let data = get_data()?;
    let outcome1 = captcha_v1(&data);
    println!("v1: {}", outcome1);
    let outcome2 = captcha_v2(&data);
    println!("v2: {}", outcome2);
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
    fn cases_v1() {
        assert_eq!(captcha_v1(&[1, 1, 2, 2]), 3);
        assert_eq!(captcha_v1(&[1, 1, 1, 1]), 4);
        assert_eq!(captcha_v1(&[1, 2, 3, 4]), 0);
        assert_eq!(captcha_v1(&[9, 1, 2, 1, 2, 1, 2, 9]), 9);
    }

    #[test]
    fn cases_v2() {
        assert_eq!(captcha_v2(&[1, 2, 1, 2]), 6);
        assert_eq!(captcha_v2(&[1, 2, 2, 1]), 0);
        assert_eq!(captcha_v2(&[1, 2, 3, 4, 2, 5]), 4);
        assert_eq!(captcha_v2(&[1, 2, 3, 1, 2, 3]), 12);
        assert_eq!(captcha_v2(&[1, 2, 1, 3, 1, 4, 1, 5]), 4);
    }
}
