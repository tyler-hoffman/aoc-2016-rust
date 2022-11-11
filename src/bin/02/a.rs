mod common;

use common::{KeyPad, Point};
use std::error::Error;
use std::fs;

const KEYPAD_LAYOUT: &str = "1 2 3
4 5 6
7 8 9";

fn main() -> Result<(), Box<dyn Error>> {
    let output = solve_for_input()?;
    print!("solution: {}\n", output);
    Ok(())
}

fn solve_for_input() -> Result<String, Box<dyn Error>> {
    let input = fs::read_to_string("src/bin/02/input.txt")?;
    let directions = common::parse(&input);

    Ok(solve(directions))
}

fn solve(lines: Vec<&str>) -> String {
    let keypad = KeyPad::from_str(KEYPAD_LAYOUT);
    let mut pos = keypad.char_point("5").expect("Pretty sure this exists");
    let mut output = String::new();

    for line in lines {
        for ch in line.bytes() {
            let next_pos = match ch {
                b'U' => Point {
                    x: pos.x,
                    y: pos.y - 1,
                },
                b'D' => Point {
                    x: pos.x,
                    y: pos.y + 1,
                },
                b'L' => Point {
                    x: pos.x - 1,
                    y: pos.y,
                },
                b'R' => Point {
                    x: pos.x + 1,
                    y: pos.y,
                },
                _ => panic!("We can't get here"),
            };

            if keypad.has_point(&next_pos) {
                pos = next_pos;
            }
        }
        match keypad.key_at_point(&pos) {
            Some(key) => output.push_str(key),
            None => panic!("Woopsie"),
        }
    }

    output
}

#[cfg(test)]
mod tests {
    use super::*;
    use common;

    #[test]
    fn case_1() {
        let input = "ULL
RRDDD
LURDL
UUUUD";
        let directions = common::parse(input);
        let output = solve(directions);

        assert_eq!(output, "1985");
    }

    #[test]
    fn case_my_input() {
        let output = solve_for_input().expect("this should work");

        assert_eq!(output, "98575");
    }
}
