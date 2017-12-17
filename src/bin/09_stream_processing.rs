extern crate advent_of_code;
#[macro_use]
extern crate failure;
extern crate itertools;

use advent_of_code::Result;

use std::fs;
use std::io::prelude::*;

fn get_data(path: &str) -> Result<Vec<u8>> {
    let mut f = fs::File::open(path)?;
    let mut v = Vec::new();
    f.read_to_end(&mut v)?;
    Ok(v)
}

#[derive(Clone, Copy, Debug)]
enum State {
    Start,
    InGroup,
    InGarbage,
    End,
}

fn process_stream(stream: &[u8]) -> Result<i32> {
    use State::*;
    let mut state = Start;
    let mut ct = 0;
    let mut depth = 0;
    let mut iter = stream.iter();
    while let Some(s) = iter.next() {
        state = match *s {
            b'{' => match state {
                Start | InGroup => { depth += 1; InGroup},
                InGarbage => InGarbage,
                End => bail!("Bad: {}", *s as char),
            },
            b'<' => match state {
                End => bail!("Bad: {}", *s as char),
                Start | InGroup => InGarbage,
                InGarbage => InGarbage
            },
            b'>' => match state {
                Start | End | InGroup => bail!("Bad: {}", *s as char),
                InGarbage => InGroup
            },
            b'}' => match state {
                Start => bail!("Bad: {}", *s as char),
                InGroup | End => { ct += depth; depth -= 1; End },
                InGarbage => InGarbage,
            },
            b'!' => match state {
                Start | End | InGroup => bail!("Bad: {}", *s as char),
                InGarbage => { iter.next().unwrap(); InGarbage }
            },
            b',' => match state {
                Start => bail!("Bad: {}", *s as char),
                End => Start,
                InGarbage | InGroup => state,
            },
            b'\n' => state,
            _ => match state {
                Start | InGroup | End => bail!("Bad: {}", *s as char),
                InGarbage => InGarbage,
            },
        };
    }
    assert_eq!(depth, 0);
    Ok(ct)
}

fn run() -> Result<()> {
    let data = get_data("data/09.txt")?;
    let outcome1 = process_stream(&data)?;
    println!("v1: {}", outcome1);
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
        let data = b"{{<a!>},{<a!>},{<a!>},{<ab>}}";
        let outcome = process_stream(data).unwrap();
        assert_eq!(outcome, 3);

        let data = b"{{<!!>},{<!!>},{<!!>},{<!!>}}";
        let outcome = process_stream(data).unwrap();
        assert_eq!(outcome, 9);
    }
}
