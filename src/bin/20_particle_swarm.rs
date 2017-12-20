extern crate advent_of_code;
extern crate failure;
extern crate regex;

use advent_of_code::Result;

use std::fs;
use std::io::prelude::*;
use std::collections::HashMap;


#[derive(Clone, Debug)]
struct Particle {
    ix: usize,
    pos: [i64; 3],
    vel: [i64; 3],
    acc: [i64; 3],
}

impl Particle {
    fn step(&mut self) {
        for ix in 0..3 {
            self.vel[ix] += self.acc[ix];
            self.pos[ix] += self.vel[ix];
        }
    }

    fn abs_accel(&self) -> i64 {
        self.acc[0].abs() + self.acc[1].abs() + self.acc[2].abs()
    }
}

fn parse(s: &str) -> Vec<Particle> {
    let rx = regex::Regex::new(r"(?x)
            p=<(-?\d+),(-?\d+),(-?\d+)>,\s
            v=<(-?\d+),(-?\d+),(-?\d+)>,\s
            a=<(-?\d+),(-?\d+),(-?\d+)>").unwrap();
    rx.captures_iter(s).enumerate().map(|(ix, cap)| {
        let pos = [
            cap.get(1).unwrap().as_str().parse::<i64>().unwrap(),
            cap.get(2).unwrap().as_str().parse::<i64>().unwrap(),
            cap.get(3).unwrap().as_str().parse::<i64>().unwrap(),
        ];
        let vel = [
            cap.get(4).unwrap().as_str().parse::<i64>().unwrap(),
            cap.get(5).unwrap().as_str().parse::<i64>().unwrap(),
            cap.get(6).unwrap().as_str().parse::<i64>().unwrap(),
        ];
        let acc = [
            cap.get(7).unwrap().as_str().parse::<i64>().unwrap(),
            cap.get(8).unwrap().as_str().parse::<i64>().unwrap(),
            cap.get(9).unwrap().as_str().parse::<i64>().unwrap(),
        ];
        Particle { ix, pos, vel, acc }
    }).collect()
}

fn get_data(path: &str) -> Result<Vec<Particle>> {
    let mut f = fs::File::open(path)?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    let pts = parse(&s);
    assert_eq!(pts.len(), 1000);
    Ok(pts)
}

fn swarm(particles: &[Particle]) -> usize {
    let itm = particles
        .iter()
        .min_by(|p1, p2| p1.abs_accel().cmp(&p2.abs_accel())).unwrap();
    itm.ix
}

fn swarm2(particles: &mut Vec<Particle>) -> usize {
    for _ in 0..100 {  // turns out to be enough
        let mut collision_map = HashMap::new();
        for (ix, p) in particles.iter().enumerate() {
            let v = collision_map.entry(p.pos).or_insert(Vec::new());
            v.push(ix)
        }
        let mut to_rm = Vec::new();
        for posn in collision_map.values() {
            if posn.len() > 1 {
                to_rm.extend(posn)
            }
        }
        to_rm.sort();
        for ix in to_rm.iter().rev() {
            particles.remove(*ix);
        }
        for p in particles.iter_mut() {
            p.step()
        }
    }
    particles.len()
}

fn run() -> Result<()> {
    let mut data = get_data("data/20.txt")?;

    let outcome = swarm(&data);
    println!("v1: {}", outcome);

    let outcome = swarm2(&mut data);
    println!("v2: {}", outcome);

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
        let data = get_data("data/19_test.txt").unwrap();
        let outcome = tubes(&data);
        assert_eq!(&outcome.0, &['A', 'B', 'C', 'D', 'E', 'F']);
        assert_eq!(outcome.1, 38);
    }
}
