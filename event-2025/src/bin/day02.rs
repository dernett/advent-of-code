use anyhow::{Result, anyhow};
use chumsky::prelude::*;
use util::InputFile;

#[derive(Debug, Copy, Clone)]
struct Range {
    low: u64,
    hi: u64,
}

impl Range {
    fn parser<'src>() -> impl Parser<'src, &'src str, Self, extra::Err<Rich<'src, char>>> {
        util::unsigned::<u64>(10)
            .then_ignore(just("-"))
            .then(util::unsigned::<u64>(10))
            .map(|(low, hi)| Self { low, hi })
    }
}

#[derive(Debug)]
struct Input {
    ranges: Vec<Range>,
}

impl Input {
    fn parse(file: &InputFile) -> Result<Self> {
        Self::parser()
            .parse(&file.contents)
            .into_result()
            .map_err(|errs| {
                file.print_diagnostics(errs);
                anyhow!("Failed to parse input file '{}'", file.path.display())
            })
    }

    fn parser<'src>() -> impl Parser<'src, &'src str, Self, extra::Err<Rich<'src, char>>> {
        Range::parser()
            .separated_by(just(","))
            .collect::<Vec<_>>()
            .map(|ranges| Self { ranges })
            .then_ignore(text::newline().or_not())
            .then_ignore(end())
    }

    fn is_invalid(id: u64) -> bool {
        let num_digits = id.max(1).ilog10() + 1;
        if num_digits.is_multiple_of(2) {
            let pow_10 = 10_u64.pow(num_digits / 2);
            id % pow_10 == id / pow_10
        } else {
            false
        }
    }

    fn is_invalid2(id: u64) -> bool {
        let num_digits = id.max(1).ilog10() + 1;
        for d in 1..=num_digits / 2 {
            if !num_digits.is_multiple_of(d) {
                continue;
            }
            let k = num_digits / d;
            // Construct pattern of `k` ones each spaced apart by `d - 1` zeros
            let pattern = (10_u64.pow(k * d) - 1) / (10_u64.pow(d) - 1);
            if id.is_multiple_of(pattern) {
                return true;
            }
        }
        false
    }

    fn part_one(&self) -> u64 {
        self.ranges
            .iter()
            .flat_map(|range| (range.low..=range.hi).filter(|x| Self::is_invalid(*x)))
            .sum()
    }

    fn part_two(&self) -> u64 {
        self.ranges
            .iter()
            .flat_map(|range| (range.low..=range.hi).filter(|x| Self::is_invalid2(*x)))
            .sum()
    }
}

fn main() -> Result<()> {
    let input_file = InputFile::read("event-2025/input/day02.txt")?;
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
    fn is_invalid_works() {
        assert!(Input::is_invalid(11));
        assert!(Input::is_invalid(22));
        assert!(Input::is_invalid(99));
        assert!(Input::is_invalid(1010));
        assert!(Input::is_invalid(1188511885));
        assert!(Input::is_invalid(222222));

        assert!(!Input::is_invalid(0));
        assert!(!Input::is_invalid(9));
        assert!(!Input::is_invalid(10));
    }

    #[test]
    fn is_invalid2_works() {
        assert!(Input::is_invalid2(11));
        assert!(Input::is_invalid2(22));
        assert!(Input::is_invalid2(99));
        assert!(Input::is_invalid2(1010));
        assert!(Input::is_invalid2(1188511885));
        assert!(Input::is_invalid2(222222));

        assert!(Input::is_invalid2(565656));
        assert!(Input::is_invalid2(824824824));
        assert!(Input::is_invalid2(2121212121));
    }

    #[test]
    fn example_works() {
        let contents = indoc! {"
            11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124
        "};
        let example = Input::parse(&contents.into()).unwrap();
        assert_eq!(example.part_one(), 1227775554);
        assert_eq!(example.part_two(), 4174379265);
    }
}
