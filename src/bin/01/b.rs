mod common;

use common::{Move, Point, PositionGetter};
use std::collections::HashSet;
use std::error::Error;
use std::fs;

fn main() -> Result<(), Box<dyn Error>> {
    let output = solve_for_input()?;
    print!("solution: {}\n", output);
    Ok(())
}

fn solve_for_input() -> Result<i32, Box<dyn Error>> {
    let input = fs::read_to_string("src/bin/01/input.txt")?;
    let directions = common::parse(&input)?;

    Ok(solve(directions))
}

fn solve(directions: Vec<Move>) -> i32 {
    let position_getter = PositionGetter::from_moves(Box::new(directions.into_iter()));
    let mut visited: HashSet<Point> = HashSet::new();

    for pos in position_getter {
        dbg!(&pos);
        if visited.contains(&pos) {
            return pos.x.abs() + pos.y.abs();
        } else {
            visited.insert(pos);
        }  
    }
    // we should never get here
    -1
}

#[cfg(test)]
mod tests {
    use super::*;
    use common;

    #[test]
    fn case_sample() {
        let directions = common::parse("R8, R4, R4, R8").expect("wat");
        let output = solve(directions);

        assert_eq!(output, 4-1);
    }

    #[test]
    fn case_my_input() {
        let output = solve_for_input().expect("wat");
        assert_eq!(output, 79);
    }
}
