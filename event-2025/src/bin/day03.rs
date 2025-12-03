use anyhow::{Result, anyhow};
use chumsky::prelude::*;
use util::InputFile;

#[derive(Debug)]
struct Input {
    banks: Vec<Vec<u64>>,
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
        text::digits(10)
            .to_slice()
            .map(|x: &str| {
                x.chars()
                    .map(|c| u64::from(c.to_digit(10).unwrap()))
                    .collect::<Vec<_>>()
            })
            .separated_by(text::newline())
            .allow_trailing()
            .collect::<Vec<_>>()
            .map(|banks| Self { banks })
    }

    fn max_joltage(ratings: &[u64], digits: usize) -> u64 {
        if digits == 0 {
            return 0;
        }
        // Use a greedy approach. We want the largest digit the farthest to the left.
        // This means that if there isn't enough digits to the right, then compute a
        // "deficit", which is the smallest number of digits to the left we need so that
        // deficit + need_right + 1 == digits.
        for d in (0..=9).rev() {
            if let Some(pos) = ratings.iter().position(|&x| x == d) {
                let num_right = ratings.len() - pos - 1;
                let deficit = (digits - 1).saturating_sub(num_right);
                let need_right = (digits - 1) - deficit;
                let left = Self::max_joltage(&ratings[..pos], deficit);
                let right = Self::max_joltage(&ratings[pos + 1..], need_right);
                return 10u64.pow(need_right.try_into().unwrap()) * (10 * left + d) + right;
            }
        }
        unreachable!("ratings should have digits");
    }

    fn part_one(&self) -> u64 {
        self.banks
            .iter()
            .map(|bank| Self::max_joltage(bank, 2))
            .sum()
    }

    fn part_two(&self) -> u64 {
        self.banks
            .iter()
            .map(|bank| Self::max_joltage(bank, 12))
            .sum()
    }
}

fn main() -> Result<()> {
    let input_file = InputFile::read("event-2025/input/day03.txt")?;
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
    fn example_works() {
        let contents = indoc! {"
            987654321111111
            811111111111119
            234234234234278
            818181911112111
        "};
        let example = Input::parse(&contents.into()).unwrap();
        assert_eq!(example.part_one(), 357);
        assert_eq!(example.part_two(), 3121910778619);
    }
}
