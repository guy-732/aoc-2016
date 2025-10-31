use fnv::FnvHashMap;
use itertools::Itertools;
use ndarray::{Array2, Axis, ShapeError};

#[aoc_generator(day06)]
fn parse(input: &str) -> Result<Array2<u8>, ShapeError> {
    let mut iter = input
        .lines()
        .map(str::trim)
        .filter(|line| !line.is_empty())
        .map(str::as_bytes);

    let mut data = iter
        .next()
        .expect("Empty input")
        .iter()
        .copied()
        .collect_vec();

    let mut nrows = 1;
    let width = data.len();

    for line in iter {
        if line.len() != width {
            panic!(
                "First line is {} characters long, but a line has {}",
                width,
                line.len()
            );
        }

        nrows += 1;
        data.extend_from_slice(line);
    }

    Array2::from_shape_vec((nrows, width), data)
}

fn occurences<I: IntoIterator<Item = u8>>(characters: I) -> FnvHashMap<u8, usize> {
    let mut occs: FnvHashMap<u8, usize> = FnvHashMap::default();

    characters
        .into_iter()
        .for_each(|b| *occs.entry(b).or_insert(0) += 1);

    occs
}

fn most_common_char<I: IntoIterator<Item = u8>>(characters: I) -> char {
    let occs = occurences(characters);

    *occs
        .iter()
        .max_by_key(|e| *e.1)
        .expect("Occurence map empty?")
        .0 as char
}

fn lest_common_char<I: IntoIterator<Item = u8>>(characters: I) -> char {
    let occs = occurences(characters);

    *occs
        .iter()
        .min_by_key(|e| *e.1)
        .expect("Occurence map empty?")
        .0 as char
}

#[aoc(day06, part1)]
fn part1(input: &Array2<u8>) -> String {
    input
        .axis_iter(Axis(1))
        .map(|row| most_common_char(row.iter().copied()))
        .collect()
}

#[aoc(day06, part2)]
fn part2(input: &Array2<u8>) -> String {
    input
        .axis_iter(Axis(1))
        .map(|row| lest_common_char(row.iter().copied()))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "
        eedadn
        drvtee
        eandsr
        raavrd
        atevrs
        tsrnev
        sdttsa
        rasrtv
        nssdts
        ntnada
        svetve
        tesnvt
        vntsnd
        vrdear
        dvrsen
        enarar";

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(EXAMPLE).expect("Failed to parse")), "easter");
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse(EXAMPLE).expect("Failed to parse")), "advent");
    }
}
