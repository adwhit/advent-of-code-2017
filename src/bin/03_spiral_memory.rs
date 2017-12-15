#![feature(generators, generator_trait, conservative_impl_trait)]

use std::ops::{Generator, GeneratorState};

#[derive(Clone, Copy, Debug)]
enum Direction {
    Right,
    Up,
    Left,
    Down
}

impl Direction {
    fn next(self) -> Direction {
        use Direction::*;
        match self {
            Right => Up,
            Up => Left,
            Left => Down,
            Down => Right
        }
    }
}

fn coord() -> impl Generator<Yield=(i32, i32), Return=()> {
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
    }
}

fn spiral(val: u32) -> u32 {
    let mut gen = coord();
    for _ in 0..(val - 1) {
        gen.resume();
    }
    match gen.resume() {
        GeneratorState::Yielded((x, y)) => x.abs() as u32 + y.abs() as u32,
        _ => unreachable!()
    }
}

fn main() {
    println!("v1: {}", spiral(277678))
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
}
