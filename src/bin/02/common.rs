use std::collections::HashMap;

#[derive(PartialEq, Eq, Hash, Debug)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

pub struct KeyPad {
    keys: HashMap<Point, String>,
}

impl KeyPad {
    pub fn char_point(&self, ch: &str) -> Result<Point, &'static str> {
        for (point, key) in &self.keys {
            if key == ch {
                return Ok(Point {
                    x: point.x,
                    y: point.y,
                });
            }
        }
        Err("key not found")
    }

    pub fn has_point(&self, point: &Point) -> bool {
        self.keys.contains_key(point)
    }

    pub fn key_at_point(&self, point: &Point) -> Option<&str> {
        match self.keys.get(point) {
            Some(key) => Some(&key[..]),
            None => None,
        }
    }

    pub fn from_str(s: &str) -> KeyPad {
        let mut keys: HashMap<Point, String> = HashMap::new();
        let lines = s.lines();

        for (y, line) in lines.enumerate() {
            for (x, ch) in line.chars().enumerate() {
                if x % 2 == 0 {
                    let point = Point {
                        x: (x / 2) as i32,
                        y: y as i32,
                    };
                    keys.insert(point, String::from(ch));
                }
            }
        }

        return KeyPad { keys };
    }
}

pub fn parse(input: &str) -> Vec<&str> {
    input.lines().collect()
}
