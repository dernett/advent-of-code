use anyhow::{Result, anyhow};
use chumsky::prelude::*;
use util::InputFile;

#[derive(Debug)]
struct Input {
    rows: Vec<String>,
    operators: String,
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
        let row = util::unsigned::<u64>(10)
            .padded_by(just(' ').repeated())
            .repeated()
            .at_least(1)
            .to_slice()
            .map(String::from);

        let rows = row.separated_by(text::newline()).collect::<Vec<_>>();

        let operators = one_of("+*")
            .padded_by(just(' ').repeated())
            .repeated()
            .at_least(1)
            .to_slice()
            .map(String::from);

        rows.then_ignore(text::newline())
            .then(operators)
            .map(|(rows, operators)| Self { rows, operators })
            .then_ignore(text::newline().or_not())
    }

    fn part_one(&self) -> u64 {
        let mut total = 0;
        let mut numbers: Vec<_> = self
            .rows
            .iter()
            .map(|row| row.split_whitespace().map(|x| x.parse().unwrap()))
            .collect();
        let ops = self.operators.chars().filter(|c| !c.is_whitespace());

        for op in ops {
            let f = match op {
                '+' => |x: u64, y: u64| x + y,
                '*' => |x: u64, y: u64| x * y,
                _ => unreachable!(),
            };
            let answer = numbers
                .iter_mut()
                .filter_map(|iter| iter.next())
                .reduce(f)
                .expect("row should not be empty");

            total += answer;
        }

        total
    }

    fn part_two(&self) -> u64 {
        let mut ans = 0;
        let mut total = 0;
        let mut op = '+';
        let mut columns: Vec<_> = self.rows.iter().map(|row| row.chars()).collect();

        for c in self.operators.chars() {
            match c {
                '+' => {
                    total = 0;
                    op = c;
                }
                '*' => {
                    total = 1;
                    op = c;
                }
                _ => (),
            }

            let num: u64 = columns
                .iter_mut()
                .filter_map(|iter| iter.next())
                .filter(|c| c.is_ascii_digit())
                .fold(0, |sum, c| 10 * sum + u64::from(c.to_digit(10).unwrap()));

            // Assume that zero columns correspond to separators
            if num == 0 {
                ans += total;
            } else {
                total = match op {
                    '+' => total + num,
                    '*' => total * num,
                    _ => unreachable!(),
                };
            }
        }

        ans += total;

        ans
    }
}

fn main() -> Result<()> {
    let input_file = InputFile::read("event-2025/input/day06.txt")?;
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
            123 328  51 64 
             45 64  387 23 
              6 98  215 314
            *   +   *   +  
        "};
        let example = Input::parse(&contents.into()).unwrap();
        assert_eq!(example.part_one(), 4277556);
        assert_eq!(example.part_two(), 3263827);
    }
}
