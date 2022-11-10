mod common;

use common::{Move, PositionGetter};
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
    let final_point = position_getter
        .last()
        .expect("We should have something here");

    final_point.x.abs() + final_point.y.abs()
}

#[cfg(test)]
mod tests {
    use super::*;
    use common;

    #[test]
    fn case_1() {
        let directions = common::parse("R2, L3").expect("wat");
        let output = solve(directions);

        assert_eq!(output, 5);
    }

    #[test]
    fn case_2() {
        let directions = common::parse("R2, R2, R2").expect("wat");
        let output = solve(directions);

        assert_eq!(output, 2);
    }

    #[test]
    fn case_3() {
        let directions = common::parse("R5, L5, R5, R3").expect("wat");
        let output = solve(directions);

        assert_eq!(output, 12);
    }

    #[test]
    fn case_my_input() {
        let output = solve_for_input().expect("wat");
        assert_eq!(output, 226);
    }
}
