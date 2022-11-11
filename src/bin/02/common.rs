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
                if ch != ' ' {
                    let point = Point {
                        x: (x / 2) as i32,
                        y: y as i32,
                    };
                    print!("{}, {}: {}\n", point.x, point.y, ch);
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

pub fn solve(lines: Vec<&str>, keypad: KeyPad) -> String {
    let mut pos = keypad.char_point("5").expect("Pretty sure this exists");
    print!("5: {}, {}\n", pos.x, pos.y);
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
