#![feature(generators, generator_trait, conservative_impl_trait, inclusive_range_syntax,
           never_type)]

use std::ops::{Generator, GeneratorState};
use std::collections::HashMap;

#[derive(Clone, Copy, Debug)]
enum Direction {
    Right,
    Up,
    Left,
    Down,
}

impl Direction {
    fn next(self) -> Direction {
        use Direction::*;
        match self {
            Right => Up,
            Up => Left,
            Left => Down,
            Down => Right,
        }
    }
}

fn coord() -> impl Generator<Yield = (i32, i32), Return = !> {
    use Direction::*;
    || {
        yield (0, 0);
        let (mut x, mut y) = (0, 0);
        let mut dirn = Right;
        for i in 1.. {
            for _ in 0..2 {
                for _ in 0..i {
                    match dirn {
                        Right => x += 1,
                        Up => y += 1,
                        Left => x -= 1,
                        Down => y -= 1,
                    }
                    yield (x, y)
                }
                dirn = dirn.next();
            }
        }
        unreachable!()
    }
}

fn spiral(val: u32) -> u32 {
    let mut gen = coord();
    for _ in 0..(val - 1) {
        gen.resume();
    }
    let GeneratorState::Yielded((x, y)) = gen.resume();
    x.abs() as u32 + y.abs() as u32
}

fn coord2() -> impl Generator<Yield = u32, Return = !> {
    || {
        let mut gen = coord();
        let mut map = HashMap::new();
        for ix in 1.. {
            let result = gen.resume();
            let GeneratorState::Yielded((x, y)) = result;
            let mut score = 0;
            for i in -1..=1 {
                for j in -1..=1 {
                    if let Some(score_) = map.get(&(x + i, y + j)) {
                        score += score_
                    }
                }
            }
            if ix == 1 {
                score = 1
            }
            map.insert((x, y), score);
            yield score;
        }
        unreachable!()
    }
}

fn spiral2(val: u32) -> u32 {
    let mut gen = coord2();
    for _ in 0.. {
        let GeneratorState::Yielded(r) = gen.resume();
        if r > val {
            return r;
        }
    }
    unreachable!()
}

fn main() {
    println!("v1: {}", spiral(277678));
    println!("v1: {}", spiral2(277678))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cases_v1() {
        assert_eq!(spiral(1), 0);
        assert_eq!(spiral(12), 3);
        assert_eq!(spiral(23), 2);
        assert_eq!(spiral(1024), 31);
    }

    #[test]
    fn cases_v2() {
        let mut gen = coord2();
        for v in &[1, 1, 2, 4, 5, 10, 11, 23, 25, 26] {
            let GeneratorState::Yielded(x) = gen.resume();
            assert_eq!(*v, x);
        }
    }
}
