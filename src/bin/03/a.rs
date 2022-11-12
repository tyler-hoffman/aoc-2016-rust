mod common;

use std::error::Error;
use std::fs;

fn main() -> Result<(), Box<dyn Error>> {
    let output = solve_for_input()?;
    print!("solution: {}\n", output);
    Ok(())
}

fn solve_for_input() -> Result<usize, Box<dyn Error>> {
    let input = fs::read_to_string("src/bin/03/input.txt")?;
    let directions = parse(&input);

    Ok(common::solve(directions))
}

fn parse(input: &str) -> Vec<(u32, u32, u32)> {
    let mut output: Vec<(u32, u32, u32)> = vec![];
    for line in input.lines() {
        let parts: Vec<u32> = line
            .split_whitespace()
            .map(|x| x.parse::<u32>().unwrap())
            .collect();
        output.push((parts[0], parts[1], parts[2]));
    }
    output
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_parse() {
        let output = parse(
            "  330  143  338
  769  547   83",
        );

        assert_eq!(output, vec!((330, 143, 338), (769, 547, 83)));
    }

    #[test]
    fn case_my_input() {
        let output = solve_for_input().unwrap();

        assert_eq!(output, 917);
    }
}
