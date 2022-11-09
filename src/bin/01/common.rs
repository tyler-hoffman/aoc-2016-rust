#[derive(Debug)]
#[derive(PartialEq)]
pub enum Direction {
  Right(i32),
  Left(i32),
}

pub fn parse(input: &str) -> Result<Vec<Direction>, &'static str> {
    input
        .split(", ")
        .map(|x| {
          let start = &x[..1];
          let end = &x[1..];
          let dist = end.parse::<i32>().unwrap();

          match start {
              "L" => Ok(Direction::Left(dist)),
              "R" => Ok(Direction::Right(dist)),
              _ => return Err("Invalid char found"),
          }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parsing_works() {
        let input = "L4, R-22";
        let output = parse(input).expect("wat");
        let expected = vec![Direction::Left(4), Direction::Right(-22)];

        assert_eq!(expected, output);
    }
}
