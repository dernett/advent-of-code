use std::collections::HashMap;

use anyhow::{Result, anyhow};
use chumsky::prelude::*;
use util::InputFile;

#[derive(Debug)]
struct Input {
    lines: HashMap<String, Vec<String>>,
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
        let name = one_of('a'..='z')
            .repeated()
            .at_least(1)
            .to_slice()
            .map(String::from);
        let line = name.clone().then_ignore(just(':').padded()).then(
            name.separated_by(just(' ').repeated().at_least(1))
                .collect::<Vec<_>>(),
        );
        line.separated_by(text::newline())
            .allow_trailing()
            .collect::<Vec<_>>()
            .map(|lines| Self {
                lines: lines.into_iter().collect(),
            })
    }

    fn search<'a>(&'a self, curr: &'a str, memo: &mut HashMap<&'a str, usize>) -> usize {
        if curr == "out" {
            return 1;
        }

        if let Some(val) = memo.get(curr) {
            return *val;
        }

        // TODO: Replace this with iterator. Had trouble with closure.
        let mut total = 0;
        for s in self.lines.get(curr).unwrap() {
            total += self.search(s, memo);
        }

        assert!(memo.insert(curr, total).is_none());

        total
    }

    fn part_one(&self) -> usize {
        let mut memo = HashMap::new();
        self.search("you", &mut memo)
    }

    fn search2<'a>(
        &'a self,
        curr: &'a str,
        seen_fft: bool,
        seen_dac: bool,
        memo: &mut HashMap<(&'a str, bool, bool), usize>,
    ) -> usize {
        if curr == "out" {
            return usize::from(seen_fft && seen_dac);
        }

        if let Some(val) = memo.get(&(curr, seen_fft, seen_dac)) {
            return *val;
        }

        // TODO: Replace this with iterator. Had trouble with closure.
        let mut total = 0;
        for s in self.lines.get(curr).unwrap() {
            total += self.search2(
                s,
                seen_fft || (curr == "fft"),
                seen_dac || (curr == "dac"),
                memo,
            );
        }

        assert!(memo.insert((curr, seen_fft, seen_dac), total).is_none());

        total
    }

    fn part_two(&self) -> usize {
        let mut memo = HashMap::new();
        self.search2("svr", false, false, &mut memo)
    }
}

fn main() -> Result<()> {
    let input_file = InputFile::read("event-2025/input/day11.txt")?;
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
            aaa: you hhh
            you: bbb ccc
            bbb: ddd eee
            ccc: ddd eee fff
            ddd: ggg
            eee: out
            fff: out
            ggg: out
            hhh: ccc fff iii
            iii: out
        "};
        let example = Input::parse(&contents.into()).unwrap();
        assert_eq!(example.part_one(), 5);

        let contents = indoc! {"
            svr: aaa bbb
            aaa: fft
            fft: ccc
            bbb: tty
            tty: ccc
            ccc: ddd eee
            ddd: hub
            hub: fff
            eee: dac
            dac: fff
            fff: ggg hhh
            ggg: out
            hhh: out
        "};
        let example = Input::parse(&contents.into()).unwrap();
        assert_eq!(example.part_two(), 2);
    }
}
