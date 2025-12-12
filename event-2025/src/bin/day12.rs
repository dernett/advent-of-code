use anyhow::{Result, anyhow};
use chumsky::prelude::*;
use util::InputFile;

#[derive(Debug)]
struct Input {
    shapes: Vec<[String; 3]>,
    regions: Vec<(usize, usize, Vec<usize>)>,
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
        let shape = one_of(".#")
            .repeated()
            .exactly(3)
            .to_slice()
            .map(String::from)
            .separated_by(text::newline())
            .collect_exactly::<[_; 3]>();

        let shapes = util::unsigned::<usize>(10)
            .then_ignore(just(':').then(text::newline()))
            .ignore_then(shape)
            .separated_by(text::newline().repeated().exactly(2))
            .allow_trailing()
            .collect::<Vec<_>>();

        let region_dim = util::unsigned::<usize>(10)
            .then_ignore(just('x'))
            .then(util::unsigned::<usize>(10));

        let region = region_dim
            .then_ignore(just(": "))
            .then(
                util::unsigned::<usize>(10)
                    .separated_by(just(' '))
                    .collect::<Vec<_>>(),
            )
            .map(|((w, h), v)| (w, h, v));

        let regions = region
            .separated_by(text::newline())
            .allow_trailing()
            .collect::<Vec<_>>();

        shapes
            .then(regions)
            .map(|(shapes, regions)| Self { shapes, regions })
    }

    fn part_one(&self) -> usize {
        let num_shapes = self.shapes.len();

        let shape_counts = self
            .shapes
            .iter()
            .map(|shape| {
                shape
                    .iter()
                    .map(|row| row.chars().filter(|&c| c == '#').count())
                    .sum::<usize>()
            })
            .collect::<Vec<_>>();

        let mut total = 0;

        for (w, h, quantities) in &self.regions {
            let count = quantities
                .iter()
                .zip(shape_counts.iter())
                .map(|(quantity, shape_count)| quantity * shape_count)
                .sum::<usize>();

            if count <= w * h && (w / 3) * (h / 3) >= num_shapes {
                total += 1;
            }
        }

        total
    }

    // fn part_two(&self) -> ! {
    //     todo!();
    // }
}

fn main() -> Result<()> {
    let input_file = InputFile::read("event-2025/input/day12.txt")?;
    let input = Input::parse(&input_file)?;

    println!("{}", input.part_one());
    // println!("{}", input.part_two());

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn example_works() {
        let contents = indoc! {"
            0:
            ###
            ##.
            ##.

            1:
            ###
            ##.
            .##

            2:
            .##
            ###
            ##.

            3:
            ##.
            ###
            ##.

            4:
            ###
            #..
            ###

            5:
            ###
            .#.
            ###

            4x4: 0 0 0 0 2 0
            12x5: 1 0 1 0 2 2
            12x5: 1 0 1 0 3 2
        "};
        let example = Input::parse(&contents.into()).unwrap();
        assert_eq!(example.part_one(), todo!());
        // assert_eq!(example.part_two(), todo!());
    }
}
