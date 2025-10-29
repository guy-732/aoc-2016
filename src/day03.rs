use std::{error::Error, str::FromStr};

use itertools::Itertools;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Triangle(i32, i32, i32);

impl Triangle {
    fn is_valid(self) -> bool {
        self.0 + self.1 > self.2 && self.1 + self.2 > self.0 && self.2 + self.0 > self.1
    }
}

impl FromStr for Triangle {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (n1, rest) = s
            .trim()
            .split_once(' ')
            .ok_or_else(|| format!("Could not parse string {s:?}"))?;
        let (n2, n3) = rest
            .trim()
            .split_once(' ')
            .ok_or_else(|| format!("Could not parse string {s:?}"))?;

        Ok(Self(
            n1.trim().parse()?,
            n2.trim().parse()?,
            n3.trim().parse()?,
        ))
    }
}

#[aoc_generator(day03)]
fn parse(input: &str) -> Result<Vec<Triangle>, Box<dyn Error>> {
    input
        .lines()
        .filter(|line| !line.is_empty())
        .map(Triangle::from_str)
        .try_collect()
}

#[aoc(day03, part1)]
fn part1(input: &[Triangle]) -> usize {
    input.iter().filter(|t| t.is_valid()).count()
}

#[aoc(day03, part2)]
fn part2(input: &[Triangle]) -> usize {
    input
        .iter()
        .tuples()
        .flat_map(|(t1, t2, t3)| {
            [
                Triangle(t1.0, t2.0, t3.0),
                Triangle(t1.1, t2.1, t3.1),
                Triangle(t1.2, t2.2, t3.2),
            ]
        })
        .filter(|t| t.is_valid())
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        assert!(!Triangle(5, 10, 25).is_valid());
        assert!(Triangle(5, 10, 7).is_valid());
    }
}
