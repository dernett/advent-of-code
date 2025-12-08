use std::collections::HashSet;

use anyhow::{Result, anyhow};
use chumsky::prelude::*;
use util::InputFile;
use util::collections::DSU;

#[derive(Debug)]
struct Input {
    boxes: Vec<(u64, u64, u64)>,
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
        // TODO: Better way of parsing this
        let line = util::unsigned::<u64>(10)
            .then_ignore(just(','))
            .then(util::unsigned::<u64>(10))
            .then_ignore(just(','))
            .then(util::unsigned::<u64>(10))
            .map(|((x, y), z)| (x, y, z));

        line.separated_by(text::newline())
            .allow_trailing()
            .collect::<Vec<_>>()
            .map(|boxes| Self { boxes })
    }

    fn box_distances(&self) -> Vec<(f64, usize, usize)> {
        let mut distances = Vec::new();
        let as_float = |i: usize| -> (f64, f64, f64) {
            let (xi, yi, zi) = self.boxes[i];
            (xi as f64, yi as f64, zi as f64)
        };

        for i in 0..self.boxes.len() {
            let (xi, yi, zi) = as_float(i);

            for j in i + 1..self.boxes.len() {
                let (xj, yj, zj) = as_float(j);
                let dx = xj - xi;
                let dy = yj - yi;
                let dz = zj - zi;
                let distance = f64::sqrt(dx * dx + dy * dy + dz * dz);

                distances.push((distance, i, j));
            }
        }

        distances
    }

    fn part_one(&self, num_connections: usize) -> usize {
        let mut distances = self.box_distances();
        distances.sort_by(|a, b| a.0.total_cmp(&b.0));

        let mut dsu = DSU::new(self.boxes.len());
        for (_, i, j) in &distances[..num_connections] {
            dsu.union(*i, *j);
        }

        let mut parents = HashSet::new();
        let mut sizes = Vec::new();
        for i in 0..self.boxes.len() {
            if parents.insert(dsu.find(i)) {
                sizes.push(dsu.set_size(i));
            }
        }

        sizes.sort_by(|a, b| b.cmp(a));

        sizes[..3].iter().product()
    }

    fn part_two(&self) -> u64 {
        let mut distances = self.box_distances();
        distances.sort_by(|a, b| a.0.total_cmp(&b.0));

        let mut dsu = DSU::new(self.boxes.len());
        for (_, i, j) in distances {
            if dsu.union(i, j) == self.boxes.len() {
                return self.boxes[i].0 * self.boxes[j].0;
            }
        }

        unreachable!();
    }
}

fn main() -> Result<()> {
    let input_file = InputFile::read("event-2025/input/day08.txt")?;
    let input = Input::parse(&input_file)?;

    dbg!(input.boxes.len());

    println!("{}", input.part_one(1000));
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
            162,817,812
            57,618,57
            906,360,560
            592,479,940
            352,342,300
            466,668,158
            542,29,236
            431,825,988
            739,650,466
            52,470,668
            216,146,977
            819,987,18
            117,168,530
            805,96,715
            346,949,466
            970,615,88
            941,993,340
            862,61,35
            984,92,344
            425,690,689
        "};
        let example = Input::parse(&contents.into()).unwrap();
        assert_eq!(example.part_one(10), 40);
        assert_eq!(example.part_two(), 25272);
    }
}
