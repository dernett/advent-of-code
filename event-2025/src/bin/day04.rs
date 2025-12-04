use anyhow::{Result, anyhow};
use chumsky::prelude::*;
use util::InputFile;

#[derive(Debug, Clone)]
struct Input {
    grid: Vec<Vec<char>>,
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
        one_of(".@")
            .repeated()
            .at_least(1)
            .collect::<Vec<_>>()
            .separated_by(text::newline())
            .allow_trailing()
            .collect::<Vec<_>>()
            .map(|grid| Self { grid })
    }

    fn neighbors(&self, row: usize, col: usize) -> [Option<char>; 8] {
        let get = |dr: isize, dc: isize| -> Option<char> {
            let r: usize = (row as isize + dr).try_into().ok()?;
            let c: usize = (col as isize + dc).try_into().ok()?;
            self.grid.get(r)?.get(c).copied()
        };

        [
            get(-1, -1),
            get(-1, 0),
            get(-1, 1),
            get(0, -1),
            get(0, 1),
            get(1, -1),
            get(1, 0),
            get(1, 1),
        ]
    }

    fn removable_roll_positions(&self) -> Vec<(usize, usize)> {
        let mut positions = Vec::new();

        for row in 0..self.grid.len() {
            for col in 0..self.grid[row].len() {
                if self.grid[row][col] == '@' {
                    let num_adjacent = self
                        .neighbors(row, col)
                        .iter()
                        .filter(|&x| matches!(x, Some('@')))
                        .count();

                    if num_adjacent < 4 {
                        positions.push((row, col));
                    }
                }
            }
        }

        positions
    }

    fn part_one(&self) -> usize {
        self.removable_roll_positions().len()
    }

    fn part_two(&self) -> usize {
        let mut total = 0;
        let mut clone = self.clone();

        loop {
            let positions = clone.removable_roll_positions();

            if positions.is_empty() {
                break;
            }

            total += positions.len();

            for (row, col) in positions {
                clone.grid[row][col] = '.';
            }
        }

        total
    }
}

fn main() -> Result<()> {
    let input_file = InputFile::read("event-2025/input/day04.txt")?;
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
            ..@@.@@@@.
            @@@.@.@.@@
            @@@@@.@.@@
            @.@@@@..@.
            @@.@@@@.@@
            .@@@@@@@.@
            .@.@.@.@@@
            @.@@@.@@@@
            .@@@@@@@@.
            @.@.@@@.@.
        "};
        let example = Input::parse(&contents.into()).unwrap();
        // println!("{:?}", example);
        assert_eq!(example.part_one(), 13);
        assert_eq!(example.part_two(), 43);
    }
}
