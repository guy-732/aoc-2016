use fnv::FnvHashSet;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Position(isize, isize);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Direction {
    fn turn_right(self) -> Self {
        match self {
            Self::Up => Self::Right,
            Self::Right => Self::Down,
            Self::Down => Self::Left,
            Self::Left => Self::Up,
        }
    }

    fn turn_left(self) -> Self {
        match self {
            Self::Up => Self::Left,
            Self::Right => Self::Up,
            Self::Down => Self::Right,
            Self::Left => Self::Down,
        }
    }
}

impl Position {
    fn new_pos(mut self, direction: Direction, distance: isize) -> Self {
        match direction {
            Direction::Up => self.0 -= distance,
            Direction::Down => self.0 += distance,
            Direction::Left => self.1 -= distance,
            Direction::Right => self.1 += distance,
        };

        self
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Turn {
    Left,
    Right,
}

fn parse_input_part(mut part: &str) -> (Turn, isize) {
    part = part.trim();
    if let Some(right) = part.strip_prefix('R') {
        return (Turn::Right, right.parse().expect("Parse num"));
    }

    if let Some(left) = part.strip_prefix('L') {
        return (Turn::Left, left.parse().expect("Parse num"));
    }

    panic!("Cannot parse part of input {:?}", part);
}

#[aoc_generator(day01)]
fn parse(input: &str) -> Vec<(Turn, isize)> {
    input.split(',').map(parse_input_part).collect()
}

#[aoc(day01, part1)]
fn part1(input: &[(Turn, isize)]) -> usize {
    let mut position = Position(0, 0);
    let mut direction = Direction::Up;

    for &(turn, distance) in input {
        direction = match turn {
            Turn::Left => direction.turn_left(),
            Turn::Right => direction.turn_right(),
        };

        position = position.new_pos(direction, distance);
    }

    position.0.unsigned_abs() + position.1.unsigned_abs()
}

#[aoc(day01, part2)]
fn part2(input: &[(Turn, isize)]) -> usize {
    let mut previous_pos = FnvHashSet::default();
    let mut position = Position(0, 0);
    let mut direction = Direction::Up;

    previous_pos.insert(position);

    for &(turn, distance) in input.iter().cycle() {
        direction = match turn {
            Turn::Left => direction.turn_left(),
            Turn::Right => direction.turn_right(),
        };

        for _ in 0..distance {
            position = position.new_pos(direction, 1);
            if !previous_pos.insert(position) {
                return position.0.unsigned_abs() + position.1.unsigned_abs();
            }
        }
    }

    unreachable!("Infinite loop");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse("R2, L3")), 5);
        assert_eq!(part1(&parse("R2, R2, R2")), 2);
        assert_eq!(part1(&parse("R5, L5, R5, R3")), 12);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse("R8, R4, R4, R8")), 4);
    }
}
