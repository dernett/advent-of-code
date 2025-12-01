use std::cmp::Ordering;

use anyhow::{Result, anyhow};
use chumsky::prelude::*;
use itertools::Itertools;
use util::InputFile;

#[derive(Debug, Copy, Clone)]
enum Direction {
    Left,
    Right,
}

impl TryFrom<char> for Direction {
    type Error = anyhow::Error;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'L' => Ok(Direction::Left),
            'R' => Ok(Direction::Right),
            _ => Err(anyhow!("Invalid direction character '{}'", value)),
        }
    }
}

#[derive(Debug, Copy, Clone)]
struct Rotation {
    direction: Direction,
    distance: i32,
}

impl Rotation {
    fn parser<'src>() -> impl Parser<'src, &'src str, Self, extra::Err<Rich<'src, char>>> {
        one_of("LR")
            .map(Direction::try_from)
            .unwrapped()
            .then(util::unsigned::<i32>(10))
            .map(|(direction, distance)| Self {
                direction,
                distance,
            })
    }

    fn amount(&self) -> i32 {
        match self.direction {
            Direction::Left => -self.distance,
            Direction::Right => self.distance,
        }
    }
}

#[derive(Debug)]
struct Input {
    rotations: Vec<Rotation>,
}

impl Input {
    const DIAL_START: i32 = 50;
    const DIAL_LEN: i32 = 100;

    fn parser<'src>() -> impl Parser<'src, &'src str, Self, extra::Err<Rich<'src, char>>> {
        Rotation::parser()
            .separated_by(text::newline())
            .allow_trailing()
            .collect::<Vec<_>>()
            .map(|rotations| Self { rotations })
    }

    fn parse(file: &InputFile) -> Result<Self> {
        Self::parser()
            .parse(&file.contents)
            .into_result()
            .map_err(|errs| {
                file.print_diagnostics(errs);
                anyhow!("Failed to parse input file '{}'", file.path.display())
            })
    }

    fn part_one(&self) -> u32 {
        self.rotations
            .iter()
            .map(|rotation| rotation.amount())
            .scan(Self::DIAL_START, |sum, amount| {
                *sum = (*sum + amount).rem_euclid(Self::DIAL_LEN);
                Some(*sum)
            })
            .filter(|x| *x == 0)
            .count()
            .try_into()
            .unwrap()
    }

    fn part_two(&self) -> u32 {
        std::iter::once(0)
            .chain(self.rotations.iter().map(|rotation| rotation.amount()))
            .scan(Self::DIAL_START, |sum, amount| {
                *sum += amount;
                Some((
                    sum.div_euclid(Self::DIAL_LEN),
                    sum.rem_euclid(Self::DIAL_LEN),
                ))
            })
            .tuple_windows()
            .map(
                |((prev_quot, prev_rem), (curr_quot, curr_rem))| match curr_quot.cmp(&prev_quot) {
                    Ordering::Less => {
                        curr_quot.abs_diff(prev_quot)  // NOFMT
                            - u32::from(prev_rem == 0) // NOFMT
                            + u32::from(curr_rem == 0) // NOFMT
                    }
                    Ordering::Equal => u32::from(curr_rem == 0 && prev_rem != 0),
                    Ordering::Greater => curr_quot.abs_diff(prev_quot),
                },
            )
            .sum()
    }
}

fn main() -> Result<()> {
    let input_file = InputFile::read("event-2025/input/day01.txt")?;
    let input = Input::parse(&input_file)?;

    println!("{}", input.part_one());
    println!("{}", input.part_two());

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn large_amount_works() {
        let input = Input::parse(&"R1000".into()).unwrap();
        assert_eq!(input.part_two(), 10);

        let input = Input::parse(&"R1050\nL1050".into()).unwrap();
        assert_eq!(input.part_two(), 21);
    }

    #[test]
    fn at_zero_works() {
        let input = Input::parse(&"L50\nL500".into()).unwrap();
        assert_eq!(input.part_two(), 6);

        let input = Input::parse(&"R50\nR500".into()).unwrap();
        assert_eq!(input.part_two(), 6);

        let input = Input::parse(&"L25\nL25\nR25".into()).unwrap();
        assert_eq!(input.part_two(), 1);

        let input = Input::parse(&"R50\nR0".into()).unwrap();
        assert_eq!(input.part_two(), 1);
    }

    #[test]
    fn example_works() {
        let contents = indoc! {"
            L68
            L30
            R48
            L5
            R60
            L55
            L1
            L99
            R14
            L82
        "};
        let example = Input::parse(&contents.into()).unwrap();
        assert_eq!(example.part_one(), 3);
        assert_eq!(example.part_two(), 6);
    }
}
