use std::fmt::Write;

use itertools::Itertools;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Instruction {
    Up,
    Down,
    Left,
    Right,
}

impl TryFrom<u8> for Instruction {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            b'U' => Ok(Self::Up),
            b'D' => Ok(Self::Down),
            b'L' => Ok(Self::Left),
            b'R' => Ok(Self::Right),
            _ => Err(()),
        }
    }
}

#[aoc_generator(day02)]
fn parse(input: &str) -> Vec<Vec<Instruction>> {
    input
        .lines()
        .filter(|line| !line.is_empty())
        .map(str::as_bytes)
        .map(|slice| {
            slice
                .iter()
                .filter_map(|&i| Instruction::try_from(i).ok())
                .collect_vec()
        })
        .filter(|v| !v.is_empty())
        .collect()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct KeyPad(u8);

impl KeyPad {
    fn move_part1(&mut self, instr: Instruction) {
        // b'1' mod 3 is 1... funny
        match instr {
            Instruction::Up => {
                if self.0 > b'3' {
                    self.0 -= 3;
                }
            }
            Instruction::Down => {
                if self.0 < b'7' {
                    self.0 += 3;
                }
            }
            Instruction::Left => {
                if self.0 % 3 != 1 {
                    self.0 -= 1;
                }
            }
            Instruction::Right => {
                if self.0 % 3 != 0 {
                    self.0 += 1;
                }
            }
        }
    }

    fn move_part2(&mut self, instr: Instruction) {
        match instr {
            Instruction::Up => match self.0 {
                b'3' => self.0 = b'1',
                b'6'..=b'8' => self.0 -= 4,
                b'A'..=b'C' => self.0 = self.0 - b'A' + b'6',
                b'D' => self.0 = b'B',
                _ => (),
            },
            Instruction::Down => match self.0 {
                b'1' => self.0 = b'3',
                b'2'..=b'4' => self.0 += 4,
                b'6'..=b'8' => self.0 = self.0 - b'6' + b'A',
                b'B' => self.0 = b'D',
                _ => (),
            },
            Instruction::Left => match self.0 {
                b'3'..=b'4' | b'6'..=b'9' | b'B'..=b'C' => self.0 -= 1,
                _ => (),
            },
            Instruction::Right => match self.0 {
                b'2'..=b'3' | b'5'..=b'8' | b'A'..=b'B' => self.0 += 1,
                _ => (),
            },
        }
    }
}

impl Default for KeyPad {
    fn default() -> Self {
        Self(b'5')
    }
}

#[aoc(day02, part1)]
fn part1(input: &[Vec<Instruction>]) -> String {
    let mut out = String::new();
    let mut keypad = KeyPad::default();

    for line in input {
        for &instr in line {
            keypad.move_part1(instr);
        }

        write!(out, "{}", keypad.0 as char).expect("Write to string failed");
    }

    out
}

#[aoc(day02, part2)]
fn part2(input: &[Vec<Instruction>]) -> String {
    let mut out = String::new();
    let mut keypad = KeyPad::default();

    for line in input {
        for &instr in line {
            keypad.move_part2(instr);
        }

        write!(out, "{}", keypad.0 as char).expect("Write to string failed");
    }

    out
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE1: &str = "
        ULL
        RRDDD
        LURDL
        UUUUD
    ";

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(EXAMPLE1)), "1985");
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse(EXAMPLE1)), "5DB3");
    }
}
