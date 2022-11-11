#[derive(Debug, PartialEq)]
pub enum Move {
    Right(i32),
    Left(i32),
}

impl Move {
    pub fn dist(m: &Move) -> i32 {
        match m {
            Move::Right(x) => *x,
            Move::Left(x) => *x,
        }
    }
}

enum Direction {
    North,
    South,
    East,
    West,
}

#[derive(PartialEq, Eq, Hash, Debug)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

pub struct PositionGetter {
    x: i32,
    y: i32,
    moves: Box<dyn Iterator<Item = Move>>,
    direction_index: usize,
    remaining_steps: i32,
}

impl PositionGetter {
    pub fn from_moves<'a>(moves: Box<dyn Iterator<Item = Move>>) -> PositionGetter {
        PositionGetter {
            moves,
            x: 0,
            y: 0,
            direction_index: 0,
            remaining_steps: 0,
        }
    }
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

impl Iterator for PositionGetter {
    type Item = Point;

    fn next(&mut self) -> Option<Self::Item> {
        static DIRECTIONS: [Direction; 4] = [
            Direction::North,
            Direction::East,
            Direction::South,
            Direction::West,
        ];

        if self.remaining_steps == 0 {
            match self.moves.next() {
                None => {}
                Some(m) => {
                    self.remaining_steps = Move::dist(&m);
                    self.direction_index = match m {
                        Move::Right(_) => (self.direction_index + 1) % 4,
                        Move::Left(_) => (self.direction_index + 3) % 4,
                    };
                }
            }
        }

        if self.remaining_steps == 0 {
            None
        } else {
            self.remaining_steps -= 1;
            match DIRECTIONS[self.direction_index] {
                Direction::North => self.y += 1,
                Direction::South => self.y -= 1,
                Direction::East => self.x += 1,
                Direction::West => self.x -= 1,
            };

            Some(Point {
                x: self.x,
                y: self.y,
            })
        }
    }
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
