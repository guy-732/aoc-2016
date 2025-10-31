use std::str::FromStr;

use itertools::Itertools;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Address {
    supernet: Vec<String>,
    hypernet: Vec<String>,
}

impl FromStr for Address {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut iter = s.split('[');
        let mut supernet = vec![iter.next().ok_or("Empty address")?.to_string()];
        let mut hypernet = vec![];

        for part in iter {
            let (hyper, normal) = part
                .split_once(']')
                .ok_or("'[' with no corresponding ']'")?;
            hypernet.push(hyper.to_string());
            supernet.push(normal.to_string());
        }

        Ok(Self { supernet, hypernet })
    }
}

fn contains_abba(txt: &str) -> bool {
    txt.chars()
        .tuple_windows()
        .any(|(a, b, c, d)| a != b && a == d && b == c)
}

fn all_aba(txt: &str) -> impl Iterator<Item = (char, char, char)> + '_ {
    txt.chars()
        .tuple_windows()
        .filter(|&(a, b, c)| a == c && a != b)
}

fn contains_bab(txt: &str, a: char, b: char) -> bool {
    txt.chars()
        .tuple_windows::<(char, char, char)>()
        .contains(&(b, a, b))
}

impl Address {
    fn supports_tls(&self) -> bool {
        !self.hypernet.iter().any(|s| contains_abba(s))
            && self.supernet.iter().any(|s| contains_abba(s))
    }

    fn supports_ssl(&self) -> bool {
        for (a, b, _) in self.supernet.iter().flat_map(|s| all_aba(s)) {
            if self.hypernet.iter().any(|txt| contains_bab(txt, a, b)) {
                return true;
            }
        }

        false
    }
}

#[aoc_generator(day07)]
fn parse(input: &str) -> Result<Vec<Address>, &'static str> {
    input
        .lines()
        .map(str::trim)
        .filter(|line| !line.is_empty())
        .map(|line| line.parse())
        .try_collect()
}

#[aoc(day07, part1)]
fn part1(input: &[Address]) -> usize {
    input.iter().filter(|addr| addr.supports_tls()).count()
}

#[aoc(day07, part2)]
fn part2(input: &[Address]) -> usize {
    input.iter().filter(|addr| addr.supports_ssl()).count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example1() {
        assert!(Address::from_str("abba[mnop]qrst").unwrap().supports_tls());
    }

    #[test]
    fn part1_example2() {
        assert!(!Address::from_str("abcd[bddb]xyyx").unwrap().supports_tls());
    }

    #[test]
    fn part1_example3() {
        assert!(!Address::from_str("aaaa[qwer]tyui").unwrap().supports_tls());
    }

    #[test]
    fn part1_example4() {
        assert!(
            Address::from_str("ioxxoj[asdfgh]zxcvbn")
                .unwrap()
                .supports_tls()
        );
    }

    #[test]
    fn part2_example1() {
        assert!(Address::from_str("aba[bab]xyz").unwrap().supports_ssl());
    }

    #[test]
    fn part2_example2() {
        assert!(!Address::from_str("xyx[xyx]xyx").unwrap().supports_ssl());
    }

    #[test]
    fn part2_example3() {
        assert!(Address::from_str("aaa[kek]eke").unwrap().supports_ssl());
    }

    #[test]
    fn part2_example4() {
        assert!(Address::from_str("zazbz[bzb]cdb").unwrap().supports_ssl());
    }
}
