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

    Ok(solve(rooms, "north"))
}

fn solve(rooms: Vec<Room>, best_guess_at_what_we_want: &str) -> u32 {
    rooms
        .iter()
        .filter(|r| r.is_valid())
        .find(|r| r.decrypted_name().contains(best_guess_at_what_we_want))
        .map(|r| r.sector_id)
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_my_input() {
        let output = solve_for_input().unwrap();

        assert_eq!(output, 984);
    }
}
