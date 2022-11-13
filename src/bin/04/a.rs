mod common;

use common::Room;
use std::error::Error;
use std::fs;

fn main() -> Result<(), Box<dyn Error>> {
    let output = solve_for_input()?;
    print!("solution: {}\n", output);
    Ok(())
}

fn solve_for_input() -> Result<u32, Box<dyn Error>> {
    let input = fs::read_to_string("src/bin/04/input.txt")?;
    let rooms = common::parse(&input);

    Ok(solve(rooms))
}

fn solve(rooms: Vec<Room>) -> u32 {
    rooms
        .iter()
        .filter(|r| r.is_valid())
        .map(|r| r.sector_id as u32)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sample() {
        let rooms = common::parse(
            "aaaaa-bbb-z-y-x-123[abxyz]
a-b-c-d-e-f-g-h-987[abcde]
not-a-real-room-404[oarel]
totally-real-room-200[decoy]",
        );
        let output = solve(rooms);

        assert_eq!(output, 1514);
    }

    #[test]
    fn case_my_input() {
        let output = solve_for_input().unwrap();

        assert_eq!(output, 185371);
    }
}
