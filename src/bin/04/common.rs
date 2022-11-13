use regex::Regex;
use std::cmp::Ordering;
use std::collections::HashMap;

#[derive(Debug, Hash, PartialEq)]
pub struct Room {
    pub encrypted_name: String,
    pub sector_id: u32,
    pub checksum: String,
}

const FIRST_LETTER: u32 = 'a' as u32;
const DASH: u32 = '-' as u32;

impl Room {
    pub fn is_valid(self: &Self) -> bool {
        let ordered_tuples = self
            .to_ordered_tuples(self.char_frequency())
            .into_iter()
            .map(|(ch, _)| ch)
            .collect::<Vec<char>>();
        let expected = format!(
            "{}{}{}{}{}",
            ordered_tuples[0],
            ordered_tuples[1],
            ordered_tuples[2],
            ordered_tuples[3],
            ordered_tuples[4]
        );
        self.checksum == expected
    }

    pub fn decrypted_name(self: &Self) -> String {
        self.encrypted_name
            .chars()
            .map(|ch| ch as u32)
            .map(|ch| match ch {
                DASH => ' ' as u32,
                x => ((x - FIRST_LETTER + self.sector_id) % 26) + FIRST_LETTER,
            })
            .map(|ch| ch as u8 as char)
            .collect::<String>()
    }

    fn char_frequency(self: &Self) -> HashMap<char, u32> {
        let x = self.encrypted_name.chars().filter(|x| *x != '-').fold(
            HashMap::new(),
            |mut output, ch| {
                *output.entry(ch).or_insert(0) += 1;
                output
            },
        );

        x
    }

    fn to_ordered_tuples(self: &Self, freq_map: HashMap<char, u32>) -> Vec<(char, u32)> {
        let mut output: Vec<(char, u32)> = vec![];
        for (ch, freq) in freq_map {
            output.push((ch, freq));
        }

        output.sort_by(|(ch_a, freq_a), (ch_b, freq_b)| {
            if freq_a < freq_b {
                Ordering::Greater
            } else if freq_a > freq_b {
                Ordering::Less
            } else if ch_a < ch_b {
                Ordering::Less
            } else if ch_a > ch_b {
                Ordering::Greater
            } else {
                Ordering::Equal
            }
        });
        output
    }
}

pub fn parse(input: &str) -> Vec<Room> {
    let re = Regex::new(r"(.+)-(\d+)\[([a-z]+)\]").unwrap();
    input
        .trim()
        .lines()
        .map(|line| {
            let caps = re.captures(line).unwrap();
            let encrypted_name = String::from(&caps[1]);
            let sector_id = caps[2].parse::<u32>().unwrap();
            let checksum = String::from(&caps[3]);

            Room {
                encrypted_name,
                sector_id,
                checksum,
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let rooms = parse("bkwzkqsxq-tovvilokx-nozvyiwoxd-172[fstek]");

        assert_eq!(
            rooms,
            vec![Room {
                encrypted_name: String::from("bkwzkqsxq-tovvilokx-nozvyiwoxd"),
                sector_id: 172,
                checksum: String::from("fstek"),
            }]
        );
    }

    #[test]
    fn is_valid_case_1() {
        let room = Room {
            encrypted_name: String::from("aaaaa-bbb-z-y-x"),
            sector_id: 123,
            checksum: String::from("abxyz"),
        };

        assert_eq!(room.is_valid(), true);
    }

    #[test]
    fn is_valid_case_2() {
        let room = Room {
            encrypted_name: String::from("a-b-c-d-e-f-g-h"),
            sector_id: 987,
            checksum: String::from("abcde"),
        };

        assert_eq!(room.is_valid(), true);
    }

    #[test]
    fn is_valid_case_3() {
        let room = Room {
            encrypted_name: String::from("not-a-real-room"),
            sector_id: 404,
            checksum: String::from("oarel"),
        };

        assert_eq!(room.is_valid(), true);
    }

    #[test]
    fn is_valid_case_4() {
        let room = Room {
            encrypted_name: String::from("totally-real-room"),
            sector_id: 200,
            checksum: String::from("decoy"),
        };

        assert_eq!(room.is_valid(), false);
    }

    #[test]
    fn test_decrypted_name() {
        let room = Room {
            encrypted_name: String::from("qzmt-zixmtkozy-ivhz"),
            sector_id: 343,
            checksum: String::from("ignore me"),
        };
        let output = room.decrypted_name();

        assert_eq!(output, "very encrypted name");
    }
}
