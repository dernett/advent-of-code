use std::ops::RangeInclusive;

use anyhow::{Result, anyhow};
use chumsky::prelude::*;
use itertools::Itertools;
use util::InputFile;

#[derive(Debug)]
struct Input {
    fresh_id_ranges: Vec<RangeInclusive<u64>>,
    available_ids: Vec<u64>,
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
        let range = util::unsigned::<u64>(10)
            .then_ignore(just("-"))
            .then(util::unsigned::<u64>(10))
            .map(|(start, end)| start..=end);

        let ranges = range
            .then_ignore(text::newline())
            .repeated()
            .at_least(1)
            .collect::<Vec<_>>();

        let ids = util::unsigned::<u64>(10)
            .separated_by(text::newline())
            .allow_trailing()
            .collect::<Vec<_>>();

        ranges
            .then_ignore(text::newline())
            .then(ids)
            .map(|(fresh_id_ranges, available_ids)| Self {
                fresh_id_ranges,
                available_ids,
            })
    }

    fn part_one(&self) -> usize {
        self.available_ids
            .iter()
            .filter(|&id| self.fresh_id_ranges.iter().any(|range| range.contains(id)))
            .count()
    }

    fn part_two(&self) -> usize {
        let mut count = 0;
        let mut events = Vec::new();
        for range in &self.fresh_id_ranges {
            // It is important that opens are processed before closes
            events.push((range.start(), false));
            events.push((range.end(), true));
        }

        events.sort();

        let mut start = 0;
        let mut num_open = 0;

        for (val, is_close) in events {
            // Remember the start for the corresponding close
            if num_open == 0 {
                start = *val;
            }

            num_open += if is_close { -1 } else { 1 };

            if num_open == 0 {
                count += val - start + 1;
            }
        }

        count.try_into().unwrap()
    }
}

fn main() -> Result<()> {
    let input_file = InputFile::read("event-2025/input/day05.txt")?;
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
            3-5
            10-14
            16-20
            12-18

            1
            5
            8
            11
            17
            32
        "};
        let example = Input::parse(&contents.into()).unwrap();
        assert_eq!(example.part_one(), 3);
        assert_eq!(example.part_two(), 14);
    }
}
