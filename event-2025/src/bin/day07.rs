use std::collections::HashSet;

use anyhow::{Result, anyhow};
use chumsky::prelude::*;
use util::InputFile;

#[derive(Debug)]
struct Input {
    start: (usize, usize),
    splitters: HashSet<(usize, usize)>,
    width: usize,
    height: usize,
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
        let line = one_of(".^S").repeated().at_least(1);

        line.to_slice()
            .separated_by(text::newline())
            .allow_trailing()
            .collect::<Vec<_>>()
            .map(|lines| Input::from_lines(lines))
    }

    fn from_lines(lines: Vec<&str>) -> Self {
        let mut start = (0, 0);
        let mut splitters = HashSet::new();

        for (row, line) in lines.iter().enumerate() {
            for (col, c) in line.char_indices() {
                match c {
                    'S' => {
                        start = (row, col);
                    }
                    '^' => {
                        splitters.insert((row, col));
                    }
                    _ => (),
                }
            }
        }

        Self {
            start,
            splitters,
            width: lines[0].len(),
            height: lines.len(),
        }
    }

    fn part_one(&self) -> u64 {
        let mut ans = 0;
        let mut visited = HashSet::new();
        let mut stack = vec![self.start];

        while let Some(pos) = stack.pop() {
            let (row, col) = pos;

            if row >= self.height || col >= self.width || !visited.insert(pos) {
                continue;
            }

            if self.splitters.contains(&pos) {
                ans += 1;
                stack.push((row, col + 1));
                if col > 0 {
                    stack.push((row, col - 1));
                }
            } else {
                stack.push((row + 1, col));
            }
        }

        ans
    }

    fn part_two(&self) -> u64 {
        let mut num_timelines = vec![0; self.width];
        let (start_row, start_col) = self.start;
        num_timelines[start_col] = 1;

        for row in start_row + 1..=self.height {
            let mut next_num = vec![0; self.width];
            for col in 0..self.width {
                if self.splitters.contains(&(row, col)) {
                    if col > 0 {
                        next_num[col - 1] += num_timelines[col];
                    }
                    if col + 1 < self.height {
                        next_num[col + 1] += num_timelines[col];
                    }
                } else {
                    next_num[col] += num_timelines[col];
                }
            }

            num_timelines = next_num;
        }

        num_timelines.iter().sum()
    }
}

fn main() -> Result<()> {
    let input_file = InputFile::read("event-2025/input/day07.txt")?;
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
            .......S.......
            ...............
            .......^.......
            ...............
            ......^.^......
            ...............
            .....^.^.^.....
            ...............
            ....^.^...^....
            ...............
            ...^.^...^.^...
            ...............
            ..^...^.....^..
            ...............
            .^.^.^.^.^...^.
            ...............
        "};
        let example = Input::parse(&contents.into()).unwrap();
        assert_eq!(example.part_one(), 21);
        assert_eq!(example.part_two(), 40);
    }
}
