use std::{
    cmp::Reverse,
    error,
    fmt::{self, Write},
    str::FromStr,
};

use itertools::Itertools;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Room {
    name: Vec<String>,
    sector_id: u16,
    checksum: [u8; 5],
}

fn parse_serial_and_checksum(s: &str) -> Result<(u16, [u8; 5]), Box<dyn error::Error>> {
    let (sector_id, checksum) = s
        .split_once('[')
        .ok_or_else(|| format!("Part after the last '-' should be '1234[abcde]' (got {s:?})"))?;

    let checksum = checksum.trim_end_matches(']');
    Ok((
        sector_id.parse()?,
        checksum.as_bytes().try_into().map_err(|e| {
            format!("Part after the last '-' should be '1234[abcde]' (got {s:?}): {e}")
        })?,
    ))
}

impl FromStr for Room {
    type Err = Box<dyn error::Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.trim().split('-').collect_vec();
        if parts.len() < 2 {
            return Err(format!("{s:?} does not have 'abcde-123[abcde]' pattern").into());
        }

        let last = parts
            .pop()
            .expect("parts.len() >= 2, should be able to pop");
        let (sector_id, checksum) = parse_serial_and_checksum(last)?;
        let name = parts.iter().map(|s| s.to_string()).collect();

        Ok(Self {
            name,
            sector_id,
            checksum,
        })
    }
}

impl fmt::Display for Room {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for name in self.name.iter() {
            write!(f, "{name}-")?;
        }

        write!(f, "{}[", self.sector_id)?;

        for chk in self.checksum {
            write!(f, "{}", chk as char)?;
        }

        write!(f, "]")
    }
}

#[aoc_generator(day04)]
fn parse(input: &str) -> Result<Vec<Room>, Box<dyn error::Error>> {
    input
        .lines()
        .filter_map(|l| {
            let l = l.trim();
            if l.is_empty() { None } else { Some(l) }
        })
        .map(str::parse)
        .try_collect()
}

fn rotate(target: u8, by: u8) -> u8 {
    let mut flattened = target - b'a';
    flattened += by;
    flattened %= 26;
    flattened + b'a'
}

impl Room {
    fn is_real_room(&self) -> bool {
        let mut occurences = [0; 256];
        for part in self.name.iter() {
            for &c in part.as_bytes() {
                occurences[c as usize] += 1;
            }
        }

        let mut order = Vec::new();
        for (ch, occ) in occurences.into_iter().enumerate() {
            if occ <= 0 {
                continue;
            }

            order.push((occ as u32, ch as u8));
        }

        if order.len() < 5 {
            return false;
        }

        order.sort_unstable_by_key(|el| (Reverse(el.0), el.1));

        order
            .into_iter()
            .map(|e| e.1)
            .zip(self.checksum)
            .all(|(computed, checksum)| computed == checksum)
    }

    fn decrypt(&self) -> Vec<String> {
        let rot = (self.sector_id % 26) as u8;

        self.name
            .iter()
            .map(|part| part.bytes().map(|b| rotate(b, rot) as char).collect())
            .collect()
    }
}

#[aoc(day04, part1)]
fn part1(input: &[Room]) -> u64 {
    input
        .iter()
        .filter(|r| r.is_real_room())
        .map(|r| r.sector_id as u64)
        .sum()
}

#[aoc(day04, part2)]
fn part2(input: &[Room]) -> String {
    let mut buf = String::new();

    for (room, decrypted) in input
        .iter()
        .filter(|room| room.is_real_room())
        .map(|room| (room, room.decrypt().iter().join(" ")))
    {
        writeln!(buf, "{:5}: {}", room.sector_id, decrypted).expect("Write to string failed");
    }

    buf
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE1: &str = "aaaaa-bbb-z-y-x-123[abxyz]";
    const EXAMPLE2: &str = "a-b-c-d-e-f-g-h-987[abcde]";
    const EXAMPLE3: &str = "not-a-real-room-404[oarel]";
    const EXAMPLE4: &str = "totally-real-room-200[decoy]";

    #[test]
    fn part1_example1() {
        assert!(Room::from_str(EXAMPLE1).unwrap().is_real_room());
    }

    #[test]
    fn part1_example2() {
        assert!(Room::from_str(EXAMPLE2).unwrap().is_real_room());
    }

    #[test]
    fn part1_example3() {
        assert!(Room::from_str(EXAMPLE3).unwrap().is_real_room());
    }

    #[test]
    fn part1_example4() {
        assert!(!Room::from_str(EXAMPLE4).unwrap().is_real_room());
    }

    #[test]
    fn part2_example() {
        assert_eq!(
            Room::from_str("qzmt-zixmtkozy-ivhz-343[abcde]")
                .unwrap()
                .decrypt(),
            vec!["very", "encrypted", "name"]
        );
    }
}
