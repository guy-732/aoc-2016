use std::fmt::Write;

#[aoc(day05, part1)]
fn part1(input: &str) -> String {
    let mut out = String::new();
    let mut i = 0_u64;

    while out.len() < 8 {
        let digest = md5::compute(format!("{input}{i}"));
        i += 1;

        if digest.0[..2].iter().any(|&d| d != 0) || (digest.0[2] & 0xF0) != 0 {
            continue;
        }

        write!(out, "{:x}", digest.0[2] & 0x0F).expect("String write error");
    }

    out
}

#[aoc(day05, part2)]
fn part2(input: &str) -> String {
    let mut out = ['*'; 8];
    let mut i = 0_u64;

    while out.contains(&'*') {
        let digest = md5::compute(format!("{input}{i}"));
        i += 1;

        if digest.0[..2].iter().any(|&d| d != 0) || (digest.0[2] & 0xF0) != 0 {
            continue;
        }

        let pos = digest.0[2] as usize & 0x0F;
        if pos < 8 && out[pos] == '*' {
            out[pos] = {
                let b = (digest.0[3] & 0xF0) >> 4;
                match b {
                    0x0..=0x9 => (b + b'0') as char,
                    0xa..=0xf => (b - 0xa + b'a') as char,
                    _ => '*',
                }
            }
        }
    }

    out.iter().collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore = "Brute force MD5"]
    fn part1_example() {
        assert_eq!(part1("abc"), "18f47a30");
    }

    #[test]
    #[ignore = "Brute force MD5"]
    fn part2_example() {
        assert_eq!(part2("abc"), "05ace8e3");
    }
}
