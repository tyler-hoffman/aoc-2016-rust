#[derive(Debug, PartialEq)]
pub enum Move {
    Right(i32),
    Left(i32),
}

enum Direction {
    North,
    South,
    East,
    West,
}

pub fn parse(input: &str) -> Result<Vec<Move>, &'static str> {
    input
        .split(", ")
        .map(|x| {
            let start = &x[..1];
            let end = &x[1..];
            let dist = end.parse::<i32>().unwrap();

            match start {
                "L" => Ok(Move::Left(dist)),
                "R" => Ok(Move::Right(dist)),
                _ => return Err("Invalid char found"),
            }
        })
        .collect()
}

pub fn moves_dist(moves: Vec<Move>) -> i32 {
    let mut x = 0;
    let mut y = 0;
    let mut direction_index = 0;
    let directions = [Direction::North, Direction::East, Direction::South, Direction::West];

    for m in moves {
        let dist = match m {
            Move::Right(x) => x,
            Move::Left(x) => x,
        };

        direction_index = match m {
            Move::Right(_) => (direction_index + 1) % 4,
            Move::Left(_) => (direction_index + 3) % 4,
        };

        match directions[direction_index] {
            Direction::North => y += dist,
            Direction::South => y -= dist,
            Direction::East => x += dist,
            Direction::West => x -= dist,
        };
    };

    x.abs() + y.abs()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parsing_works() {
        let input = "L4, R-22";
        let output = parse(input).expect("wat");
        let expected = vec![Move::Left(4), Move::Right(-22)];

        assert_eq!(expected, output);
    }
}
