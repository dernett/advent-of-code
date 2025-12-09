use std::{
    cmp,
    collections::{HashMap, HashSet},
};

use anyhow::{Result, anyhow};
use chumsky::prelude::*;
use itertools::Itertools;
use util::InputFile;

#[derive(Debug)]
struct Input {
    red_tiles: Vec<(u64, u64)>,
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
        let line = util::unsigned::<u64>(10)
            .then_ignore(just(','))
            .then(util::unsigned::<u64>(10));

        line.separated_by(text::newline())
            .allow_trailing()
            .collect::<Vec<_>>()
            .map(|red_tiles| Self { red_tiles })
    }

    fn part_one(&self) -> u64 {
        self.red_tiles
            .iter()
            .copied()
            .tuple_combinations()
            .map(|((x1, y1), (x2, y2))| {
                let width = x1.abs_diff(x2) + 1;
                let height = y1.abs_diff(y2) + 1;
                width * height
            })
            .max()
            .expect("should have at least two tiles")
    }

    fn flood_fill(grid: &mut Vec<Vec<u8>>, x: usize, y: usize, marker: u8) {
        if grid[y][x] != b'.' {
            return;
        }

        grid[y][x] = marker;

        if x > 0 {
            Self::flood_fill(grid, x - 1, y, marker);
        }

        if x + 1 < grid.len() {
            Self::flood_fill(grid, x + 1, y, marker);
        }

        if y > 0 {
            Self::flood_fill(grid, x, y - 1, marker);
        }

        if y + 1 < grid.len() {
            Self::flood_fill(grid, x, y + 1, marker);
        }
    }

    fn part_two(&self) -> u64 {
        // Step 1: Dedup x and y coords, then sort them
        let mut x_coords: HashSet<_> = self.red_tiles.iter().copied().map(|(x, _)| x).collect();
        let mut y_coords: HashSet<_> = self.red_tiles.iter().copied().map(|(_, y)| y).collect();

        // Add extra coords so that the outside region stays connected for flood flill
        assert!(x_coords.insert(0));
        assert!(x_coords.insert(u64::MAX));

        assert!(y_coords.insert(0));
        assert!(y_coords.insert(u64::MAX));

        let x_coords: Vec<_> = x_coords.iter().copied().sorted().collect();
        let y_coords: Vec<_> = y_coords.iter().copied().sorted().collect();

        // Step 2: Create outline of red and green tiles
        let n = cmp::max(x_coords.len(), y_coords.len());
        let mut grid = vec![vec![b'.'; n]; n];

        let x_translate: HashMap<_, _> = x_coords
            .iter()
            .copied()
            .enumerate()
            .map(|(i, x)| (x, i))
            .collect();
        let y_translate: HashMap<_, _> = y_coords
            .iter()
            .copied()
            .enumerate()
            .map(|(i, y)| (y, i))
            .collect();

        let translate = |(x, y)| (*x_translate.get(&x).unwrap(), *y_translate.get(&y).unwrap());

        for (a, b) in self.red_tiles.iter().copied().circular_tuple_windows() {
            let (x1, y1) = translate(a);
            let (x2, y2) = translate(b);

            if x1 == x2 {
                // Vertical line
                let min_y = cmp::min(y1, y2);
                let max_y = cmp::max(y1, y2);

                for y in min_y..=max_y {
                    grid[y][x1] = b'#';
                }
            } else if y1 == y2 {
                // Horizontal line
                let min_x = cmp::min(x1, x2);
                let max_x = cmp::max(x1, x2);

                for x in min_x..=max_x {
                    grid[y1][x] = b'#';
                }
            } else {
                unreachable!();
            }
        }

        // Step 3: Floodfill

        // Fill the outside
        Self::flood_fill(&mut grid, 0, 0, b'~');

        // Fill the inside
        for y in 0..n {
            for x in 0..n {
                if grid[y][x] == b'.' {
                    Self::flood_fill(&mut grid, x, y, b'#');
                }
            }
        }

        // Step 4: Iterate over all pairs of red tiles and use the compressed
        // grid to determine if any sections are outside.
        let mut ans = 0;
        for (a, b) in self.red_tiles.iter().copied().tuple_combinations() {
            let (x1, y1) = translate(a);
            let (x2, y2) = translate(b);

            let min_y = cmp::min(y1, y2);
            let max_y = cmp::max(y1, y2);
            let min_x = cmp::min(x1, x2);
            let max_x = cmp::max(x1, x2);

            let mut valid = true;
            for y in min_y..=max_y {
                for x in min_x..=max_x {
                    if grid[y][x] != b'#' {
                        valid = false;
                    }
                }
            }

            if valid {
                let width = a.0.abs_diff(b.0) + 1;
                let height = a.1.abs_diff(b.1) + 1;

                ans = cmp::max(ans, width * height);
            }
        }

        ans
    }
}

fn main() -> Result<()> {
    let input_file = InputFile::read("event-2025/input/day09.txt")?;
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
            7,1
            11,1
            11,7
            9,7
            9,5
            2,5
            2,3
            7,3
        "};
        let example = Input::parse(&contents.into()).unwrap();
        assert_eq!(example.part_one(), 50);
        assert_eq!(example.part_two(), 24);
    }
}
