extern crate advent_of_code;
extern crate failure;

use advent_of_code::Result;

use std::fs;
use std::io::prelude::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Route {
    Horz,
    Vert,
    ChangeDirn,
    Crumb(char),
    None
}

impl Route {
    fn from_u8(b: u8) -> Route {
        use Route::*;
        match b {
            b'-' => Horz,
            b'|' => Vert,
            b'+' => ChangeDirn,
            b' ' => None,
            c => Crumb(c as char)
        }
    }
}


#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Dirn {
    North,
    South,
    East,
    West
}


fn get_data(path: &str) -> Result<Vec<Vec<Route>>> {
    let mut f = fs::File::open(path)?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s.lines()
        .map(|line| {
            line.bytes().map(|b| Route::from_u8(b)).collect()
        }).collect())
}

fn onward(row: usize, col: usize, dirn: Dirn) -> (usize, usize) {
    use Dirn::*;
    match dirn {
        North => (row - 1, col),
        South => (row + 1, col),
        West => (row, col - 1),
        East => (row, col + 1),
    }
}

fn tubes(data: &Vec<Vec<Route>>) -> Vec<char> {
    use Dirn::*;
    use Route::*;
    let mut row = 0;
    let mut col = data[0].iter().position(|d| *d == Route::Vert).unwrap();
    let mut dirn = South;
    let mut crumbs = Vec::new();
    loop {
        let next = match data[row][col] {
            Vert | Horz => onward(row, col, dirn),
            Crumb(c) => {crumbs.push(c); onward(row, col, dirn)},
            None => return crumbs, // we're done (or we messed up)
            ChangeDirn => {
                match dirn {
                    North | South => {
                        match data[row][col + 1] {
                            Horz | Crumb(_) => {
                                dirn = East;
                                (row, col + 1)
                            }
                            _ => match data[row][col - 1] {
                                Horz | Crumb(_) => {
                                    dirn = West;
                                    (row, col - 1)
                                }
                                _ => panic!("Something wrong at {}, {}", row, col)
                            }
                        }
                    },
                    East | West => {
                        match data[row - 1][col] {
                            Vert | Crumb(_) => {
                                dirn = North;
                                (row - 1, col)
                            }
                            _ => match data[row + 1][col] {
                                Vert | Crumb(_) => {
                                    dirn = South;
                                    (row + 1, col)
                                }
                                _ => panic!("Something wrong at {}, {}", row, col)
                            }
                        }
                    }
                }
            }
        };
        row = next.0;
        col = next.1;
    }

}


fn run() -> Result<()> {
    {
        let data = get_data("data/19.txt")?;
        let outcome = tubes(&data);
        print!("v1: ");
        for c in outcome { print!("{}", c) }
        println!()
    }

    // {
    //     let data = get_data("data/18.txt")?;
    //     let outcome = duet2(data);
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
        let data = get_data("data/19_test.txt").unwrap();
        let outcome = tubes(&data);
        assert_eq!(&outcome, &['A', 'B', 'C', 'D', 'E', 'F'])
    }

    // #[test]
    // fn cases_v2() {
    // }
}
